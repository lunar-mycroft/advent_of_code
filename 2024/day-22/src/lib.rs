use itertools::Itertools;
use tap::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    numbers: Vec<u32>,
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        Self {
            numbers: s.lines().map(str::trim).map(str::parse).try_collect()?,
        }
        .pipe(Ok)
    }
}

#[derive(Debug)]
struct Rng(u32);

impl Iterator for Rng {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let val = self.0;
        self.0 = ((self.0 << 6) ^ self.0) & 0xff_ffff;
        self.0 = ((self.0 >> 5) ^ self.0) & 0xff_ffff;
        self.0 = ((self.0 << 11) ^ self.0) & 0xff_ffff;
        Some(val)
    }
}

pub fn init_tracing() -> color_eyre::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

    let log_filter: EnvFilter = std::env::var(EnvFilter::DEFAULT_ENV)
        .map(String::leak)
        .map(|s| s as &str)
        .or_else(|err| match err {
            std::env::VarError::NotPresent => Ok("day_22=debug"),
            err @ std::env::VarError::NotUnicode(_) => Err(err),
        })?
        .parse()?;
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(log_filter)
        .with_line_number(true)
        .finish()
        .with(tracing_error::ErrorLayer::default());
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(123, 15_887_950)]
    #[case(15_887_950, 16_495_136)]
    fn test_next(#[case] before: u32, #[case] after: u32) {
        assert_eq!(Rng(before).nth(1), Some(after));
    }
}
