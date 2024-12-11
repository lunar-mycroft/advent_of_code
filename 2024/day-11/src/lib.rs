use std::collections::HashMap;

use either::Either;
use itertools::Itertools as _;
use tap::prelude::*;

#[derive(Debug, Clone)]
pub struct Puzzle {
    stones: Vec<u64>,
}

impl Puzzle {
    pub fn simulate_vec(mut self, steps: u8) -> usize {
        for _ in 0..steps {
            self.stones = self.stones.into_iter().flat_map(replace_stone).collect();
        }
        self.stones.len()
    }

    #[must_use]
    pub fn simulate_no_alloc(mut self, steps: usize) -> usize {
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

    #[must_use]
    pub fn depth_first_str(self, steps: u8) -> usize {
        #[allow(clippy::option_if_let_else)]
        fn stones_after(initial: u64, steps: u8, cache: &mut HashMap<(u64, u8), usize>) -> usize {
            match cache.get(&(initial, steps)) {
                Some(value) => *value,
                None => {
                    let value = match initial {
                        _ if steps == 0 => 1,
                        0 => stones_after(1, steps - 1, cache),
                        n if n.ilog(10) % 2 == 1 => {
                            let s = n.to_string();
                            let (a, b) = s.split_at(s.len() / 2);

                            stones_after(a.parse().expect("will always be valid"), steps - 1, cache)
                                + stones_after(
                                    b.parse().expect("will always be valid"),
                                    steps - 1,
                                    cache,
                                )
                        }
                        n => stones_after(n * 2024, steps - 1, cache),
                    };
                    cache.insert((initial, steps), value);
                    value
                }
            }
        }

        let mut cache = HashMap::new();
        self.depth_first_impl(steps, move |stone, steps| {
            stones_after(stone, steps, &mut cache)
        })
    }

    #[must_use]
    pub fn depth_first_math(self, steps: u8) -> usize {
        #[allow(clippy::option_if_let_else)]
        fn stones_after(initial: u64, steps: u8, cache: &mut HashMap<(u64, u8), usize>) -> usize {
            match cache.get(&(initial, steps)) {
                Some(value) => *value,
                None => {
                    let value = match initial {
                        _ if steps == 0 => 1,
                        0 => stones_after(1, steps - 1, cache),
                        n if n.ilog(10) % 2 == 1 => {
                            let (a, b) = crate::split_digits(n);
                            stones_after(a, steps - 1, cache) + stones_after(b, steps - 1, cache)
                        }
                        n => stones_after(n * 2024, steps - 1, cache),
                    };
                    cache.insert((initial, steps), value);
                    value
                }
            }
        }

        let mut cache = HashMap::new();
        self.depth_first_impl(steps, move |stone, steps| {
            stones_after(stone, steps, &mut cache)
        })
    }

    fn depth_first_impl(self, steps: u8, mut stones_after: impl FnMut(u64, u8) -> usize) -> usize {
        self.stones
            .into_iter()
            .map(|stone| stones_after(stone, steps))
            .sum()
    }

    #[must_use]
    pub fn breadth_first(self, steps: u8) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;
    use color_eyre::eyre::Result;

    #[test]
    fn test_simulate_vec() -> Result<()> {
        let example: Puzzle = "125 17".parse()?;
        let actual: Puzzle = common::read_input!("part1.txt").parse()?;
        assert_eq!(example.simulate_vec(25), 55_312);
        assert_eq!(actual.simulate_vec(25), 218_079);
        Ok(())
    }

    #[test]
    fn test_simulate_no_alloc() -> Result<()> {
        let example: Puzzle = "125 17".parse()?;
        let actual: Puzzle = common::read_input!("part1.txt").parse()?;
        assert_eq!(example.simulate_no_alloc(25), 55_312);
        assert_eq!(actual.simulate_no_alloc(25), 218_079);
        Ok(())
    }

    #[test]
    fn test_depth_first_str() -> Result<()> {
        let example: Puzzle = "125 17".parse()?;
        let actual: Puzzle = common::read_input!("part1.txt").parse()?;
        assert_eq!(example.clone().depth_first_str(25), 55_312);
        assert_eq!(actual.clone().depth_first_str(25), 218_079);
        assert_eq!(example.depth_first_str(75), 65_601_038_650_482);
        assert_eq!(actual.depth_first_str(75), 259_755_538_429_618);
        Ok(())
    }

    #[test]
    fn test_depth_first_math() -> Result<()> {
        let example: Puzzle = "125 17".parse()?;
        let actual: Puzzle = common::read_input!("part1.txt").parse()?;
        assert_eq!(example.clone().depth_first_math(25), 55_312);
        assert_eq!(actual.clone().depth_first_math(25), 218_079);
        assert_eq!(example.depth_first_math(75), 65_601_038_650_482);
        assert_eq!(actual.depth_first_math(75), 259_755_538_429_618);
        Ok(())
    }

    #[test]
    fn test_breadth_first() -> Result<()> {
        let example: Puzzle = "125 17".parse()?;
        let actual: Puzzle = common::read_input!("part1.txt").parse()?;
        assert_eq!(example.clone().breadth_first(25), 55_312);
        assert_eq!(actual.clone().breadth_first(25), 218_079);
        assert_eq!(example.breadth_first(75), 65_601_038_650_482);
        assert_eq!(actual.breadth_first(75), 259_755_538_429_618);
        Ok(())
    }
}
