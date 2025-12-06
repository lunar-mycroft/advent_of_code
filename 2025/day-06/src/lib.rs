use color_eyre::eyre::{bail, OptionExt};
use itertools::Itertools;
use tap::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    numbers: String,
    operations: Vec<Operator>,
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (numbers, operators) = s
            .trim()
            .rsplit_once('\n')
            .ok_or_eyre("Input doesn't contain a line break")?;
        Self {
            numbers: numbers.to_owned(),
            operations: operators
                .split_whitespace()
                .map(|s| match s {
                    "+" => Ok(Operator::Add),
                    "*" => Ok(Operator::Mul),
                    other => bail!("{other:?} is not a valid operator"),
                })
                .try_collect()?,
        }
        .pipe(Ok)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Mul,
}

impl Operator {
    #[inline]
    fn solve<E>(self, mut it: impl Iterator<Item = Result<u64, E>>) -> Result<u64, E> {
        match self {
            Self::Add => it.try_fold(0, |sum, res| Ok(sum + res?)),
            Self::Mul => it.try_fold(1, |product, res| Ok(product * res?)),
        }
    }
}

pub fn init_tracing() -> color_eyre::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

    let log_filter: EnvFilter = std::env::var(EnvFilter::DEFAULT_ENV)
        .map(String::leak)
        .map(|s| s as &str)
        .or_else(|err| match err {
            std::env::VarError::NotPresent => Ok("day_06=debug"),
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
