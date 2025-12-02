use itertools::Itertools;
use tap::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
pub struct Puzzle {
    pub ranges: Vec<Range>,
}

#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub start: u64,
    pub end: u64,
    pub start_len: u32,
    pub end_len: u32,
}

impl Range {
    fn repeat_n(&self, repetitions: u32) -> impl Iterator<Item = u64> + use<'_> {
        let begin = 10u64.pow((self.start_len / repetitions).max(1) - 1);
        (begin..)
            .map(move |n| {
                (0..repetitions)
                    .map(|p| 10u64.pow((n.checked_ilog10().unwrap_or(0) + 1) * p) * n)
                    .sum::<u64>()
            })
            .skip_while(move |&n| n < self.start)
            .take_while(move |&n| n <= self.end)
    }
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        Self {
            ranges: s
                .trim()
                .split(',')
                .map(str::trim)
                .filter_map(|s| s.trim().split_once('-'))
                .map(|(start, end)| {
                    Range {
                        start: start.parse()?,
                        end: end.parse()?,
                        start_len: start.len().try_conv()?,
                        end_len: end.len().try_conv()?,
                    }
                    .pipe(Ok::<_, color_eyre::Report>)
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
            std::env::VarError::NotPresent => Ok("day_02=debug"),
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
