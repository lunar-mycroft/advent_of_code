use std::collections::{HashMap, HashSet, VecDeque};

use color_eyre::eyre::{eyre, OptionExt};
use itertools::Itertools;
use tap::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    connections: HashMap<[u8; 3], HashSet<[u8; 3]>>,
}

impl Puzzle {
    fn count_paths(&self, from: [u8; 3], to: [u8; 3]) -> u64 {
        let (mut incoming_edges, mut queue, mut order, mut ways) =
            (HashMap::new(), VecDeque::new(), Vec::new(), HashMap::new());
        for snk in self.connections.values().flat_map(HashSet::iter).copied() {
            *incoming_edges.entry(snk).or_insert(0u32) += 1;
        }
        for node in self
            .connections
            .values()
            .flat_map(HashSet::iter)
            .chain(self.connections.keys())
            .copied()
            .filter(|node| incoming_edges.get(node).copied().unwrap_or(0) == 0)
            .unique()
        {
            queue.push_back(node);
        }
        while let Some(node) = queue.pop_front() {
            for &neighbor in self
                .connections
                .get(&node)
                .into_iter()
                .flat_map(HashSet::iter)
            {
                let n = *incoming_edges
                    .entry(neighbor)
                    .and_modify(|n| *n -= 1)
                    .or_insert(0);
                if n == 0 {
                    queue.push_back(neighbor);
                }
            }
            order.push(node);
        }
        ways.insert(from, 1u64);
        for node in &order {
            for &neighbor in self
                .connections
                .get(node)
                .into_iter()
                .flat_map(HashSet::iter)
            {
                *ways.entry(neighbor).or_insert(0) += ways.get(node).copied().unwrap_or(0);
            }
        }
        ways.get(&to).copied().unwrap_or(0)
    }
}

const OUT: [u8; 3] = [b'o', b'u', b't'];

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        Self {
            connections: s
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let (src, snk) = s.split_once(": ").ok_or_eyre("Missing source and sinks")?;

                    (
                        src.as_bytes()
                            .try_conv()
                            .map_err(|_| eyre!("{src:?} is not exactly three bytes"))?,
                        snk.split_whitespace()
                            .map(|seg| {
                                seg.as_bytes()
                                    .try_conv::<[u8; 3]>()
                                    .map_err(|_| eyre!("{seg:?} is not exactly three bytes"))
                            })
                            .try_collect::<_, HashSet<_>, _>()?,
                    )
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
