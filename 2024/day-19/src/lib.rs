use std::collections::HashMap;

use color_eyre::eyre::OptionExt;
use itertools::Itertools;
use tap::Pipe;

pub mod array_trie;
pub mod hash_trie;
pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
pub struct Puzzle {
    towels: Vec<String>,
    goals: Vec<String>,
}

#[allow(clippy::option_if_let_else)]
fn ways<'s>(goal: &'s str, towels: &'s [String], cache: &mut HashMap<&'s str, u64>) -> u64 {
    match cache.get(goal) {
        Some(n) => *n,
        None => {
            let res = towels
                .iter()
                .map(|towel| match goal.strip_prefix(towel) {
                    Some("") => 1,
                    Some(suffix) => ways(suffix, towels, cache),
                    None => 0,
                })
                .sum();
            cache.insert(goal, res);
            res
        }
    }
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        let s = s.replace('\r', "");
        let (patterns, designs) = s
            .split_once("\n\n")
            .ok_or_eyre("Couldn't seperate blocks")?;
        Self {
            towels: patterns
                .trim()
                .split(", ")
                .map(ToOwned::to_owned)
                .collect_vec(),
            goals: designs.trim().lines().map(ToOwned::to_owned).collect_vec(),
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
            std::env::VarError::NotPresent => Ok("day_19=debug"),
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
