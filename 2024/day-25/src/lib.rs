use itertools::Itertools;
use tap::prelude::*;

pub mod part1;

#[derive(Debug)]
pub struct Puzzle {
    items: Vec<u32>,
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        s.split("\n\n")
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|schematic| {
                schematic
                    .chars()
                    .filter(char::is_ascii_punctuation)
                    .map(|c| c as u32)
                    // '#' has a 1 as it's least significant bit, '.' a zero
                    .fold(0, |acc, c| (c & 1) | (acc << 1))
            })
            .collect_vec()
            .pipe(|items| Self { items })
            .pipe(Ok)
    }
}

pub fn init_tracing() -> color_eyre::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

    let log_filter: EnvFilter = std::env::var(EnvFilter::DEFAULT_ENV)
        .map(String::leak)
        .map(|s| s as &str)
        .or_else(|err| match err {
            std::env::VarError::NotPresent => Ok("day_25=debug"),
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
