use color_eyre::eyre::bail;
use itertools::Itertools;
use tap::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    banks: Vec<Bank>,
}

#[derive(Debug)]
pub struct Bank(Vec<u8>);

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        Self {
            banks: s
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .map(str::as_bytes)
                .map(|bytes| {
                    bytes
                        .iter()
                        .copied()
                        .map(|b| match b {
                            digit @ b'0'..=b'9' => Ok(digit - b'0'),
                            other => bail!("{other:?} is not an ascii digit"),
                        })
                        .try_collect::<_, Vec<_>, _>()
                        .map(Bank)
                })
                .try_collect()?,
        }
        .pipe(Ok)
    }
}

pub fn init_tracing() -> color_eyre::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

    let log_filter: EnvFilter = std::env::var(EnvFilter::DEFAULT_ENV)
        .map(String::leak)
        .map(|s| s as &str)
        .or_else(|err| match err {
            std::env::VarError::NotPresent => Ok("day_03=debug"),
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
