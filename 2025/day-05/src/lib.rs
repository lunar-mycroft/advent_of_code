use std::ops::RangeInclusive;

use color_eyre::eyre::OptionExt;
use itertools::Itertools;
use tap::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
pub struct Puzzle {
    ranges: Vec<RangeInclusive<u64>>,
    ids: Vec<u64>,
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        let (ranges, ids) = s
            .trim()
            .split_once("\n\n")
            .ok_or_eyre("Missing blank line")?;
        Self {
            ranges: {
                let mut unmerged: Vec<_> = ranges
                    .trim()
                    .lines()
                    .map(str::trim)
                    .map(|s| {
                        let (start, end) = s.split_once('-').ok_or_eyre("Missing '-'")?;
                        let (start, end): (u64, u64) = (start.parse()?, end.parse()?);
                        if start > end {
                            end..=start
                        } else {
                            start..=end
                        }
                        .pipe(Ok::<_, color_eyre::Report>)
                    })
                    .try_collect()?;
                unmerged.sort_by_key(|r| *r.start());
                let len = unmerged.len();
                unmerged.into_iter().fold(
                    Vec::<RangeInclusive<_>>::with_capacity(len),
                    |mut v, curr| {
                        match v.last_mut() {
                            Some(last) if curr.start() <= last.end() => {
                                *last = *last.start()..=*(last.end().max(curr.end()));
                            }
                            Some(_) | None => v.push(curr),
                        }
                        v
                    },
                )
            },
            ids: ids.trim().lines().map(str::parse::<u64>).try_collect()?,
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
            std::env::VarError::NotPresent => Ok("day_05=debug"),
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
