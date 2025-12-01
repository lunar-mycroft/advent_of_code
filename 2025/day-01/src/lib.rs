use itertools::Itertools;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    rotations: Vec<i16>,
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    #[allow(clippy::cast_possible_wrap, reason = "not possible with two digits")]
    fn from_str(s: &str) -> color_eyre::Result<Self> {
        s.split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| match s.as_bytes()[0] {
                b'L' => Ok(-i16::from_str(&s[1..])?),
                b'R' => Ok(i16::from_str(&s[1..])?),
                _ => color_eyre::eyre::bail!("Invalid line {s:?}"),
            })
            .try_collect()
            .map(|rotations| Self { rotations })
    }
}

pub fn init_tracing() -> color_eyre::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

    let log_filter: EnvFilter = std::env::var(EnvFilter::DEFAULT_ENV)
        .map(String::leak)
        .map(|s| s as &str)
        .or_else(|err| match err {
            std::env::VarError::NotPresent => Ok("day_01=debug"),
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
