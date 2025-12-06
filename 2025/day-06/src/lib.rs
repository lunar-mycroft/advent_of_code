use color_eyre::eyre::{bail, ensure, OptionExt};
use itertools::Itertools;
use tap::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    problems: Vec<Problem>,
}

#[derive(Debug)]
pub enum Problem {
    Add(Vec<u64>),
    Multiply(Vec<u64>),
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        let rows = s
            .trim()
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter(|s| !s.is_empty())
                    .collect_vec()
            })
            .collect_vec();
        ensure!(rows.len() > 1, "Too few rows");
        ensure!(
            rows.iter().map(Vec::len).all_equal(),
            "Non-rectangular input"
        );
        let width = rows.first().expect("Verified non-empty").len();
        Self {
            problems: (0..width)
                .map(|x| {
                    let nums: Vec<_> = rows
                        .iter()
                        .rev()
                        .skip(1)
                        .map(|row| row[x])
                        .map(str::parse::<u64>)
                        .try_collect()?;
                    match rows.last().expect("Known non-empty")[x] {
                        "+" => Problem::Add(nums).pipe(Ok),
                        "*" => Problem::Multiply(nums).pipe(Ok),
                        s => bail!("Invaild operator: {s:?}"),
                    }
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
