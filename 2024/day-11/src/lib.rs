use std::collections::HashMap;

use either::Either;
use itertools::Itertools as _;
use tap::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
pub struct Puzzle {
    stones: Vec<u64>,
}

impl Puzzle {
    fn simulate(mut self, steps: u8) -> usize {
        for _ in 0..steps {
            self.stones = self.stones.into_iter().flat_map(replace_stone).collect();
        }
        self.stones.len()
    }

    fn simulate_no_alloc(mut self, steps: usize) -> usize {
        std::iter::from_fn(move || {
            self.stones = self
                .stones
                .iter()
                .copied()
                .flat_map(replace_stone_either)
                .collect_vec();
            self.stones.clone().pipe(Some)
        })
        .nth(steps - 1)
        .expect("n must be greater than zero")
        .len()
    }

    fn depth_first(self, steps: u8, mut stones_after: impl FnMut(u64, u8) -> usize) -> usize {
        self.stones
            .into_iter()
            .map(|stone| stones_after(stone, steps))
            .sum()
    }

    fn breadth_first(self, steps: u8) -> usize {
        let mut counts = self
            .stones
            .into_iter()
            .fold(HashMap::new(), |mut counts, stone| {
                *counts.entry(stone).or_insert(0usize) += 1usize;
                counts
            });
        for _ in 0..steps {
            let mut new_counts = HashMap::new();
            for (stone, count) in counts {
                match stone {
                    0 => *new_counts.entry(1).or_insert(0usize) += count,
                    n if n.ilog(10) % 2 == 1 => {
                        let (a, b) = split_digits(n);
                        *new_counts.entry(a).or_insert(0usize) += count;
                        *new_counts.entry(b).or_insert(0usize) += count;
                    }
                    n => *new_counts.entry(n * 2024).or_insert(0usize) += count,
                }
            }
            counts = new_counts;
        }
        counts.into_values().sum()
    }
}

fn replace_stone(stone: u64) -> Vec<u64> {
    match stone {
        0 => vec![1],
        n if n.ilog(10) % 2 == 1 => {
            let s = n.to_string();
            let (a, b) = s.split_at(s.len() / 2);
            vec![
                a.parse().expect("will always be valid"),
                b.parse().expect("will always be valid"),
            ]
        }
        n => vec![n * 2024],
    }
}

#[inline]
fn replace_stone_either(stone: u64) -> impl Iterator<Item = u64> {
    match stone {
        0 => Either::Left([1]),
        n if n.ilog(10) % 2 == 1 => split_digits(n).conv::<[_; 2]>().pipe(Either::Right),
        n => Either::Left([n * 2024]),
    }
    .into_iter()
}

const fn split_digits(stone: u64) -> (u64, u64) {
    let div = 10u64.pow(stone.ilog10() / 2 + 1);
    (stone / div, stone % div)
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        Self {
            stones: s
                .split_whitespace()
                .map(str::trim)
                .map(str::parse::<u64>)
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
            std::env::VarError::NotPresent => Ok("day_11=debug"),
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
