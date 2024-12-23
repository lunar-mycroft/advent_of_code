use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use tap::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    edges: Vec<Edge>,
}

impl Puzzle {
    fn nodes(&self) -> impl Iterator<Item = &str> {
        self.edges
            .iter()
            .flat_map(|conn| [conn.from.as_str(), conn.to.as_str()])
            .unique()
    }

    fn all_edges(&self) -> impl Iterator<Item = EdgeRef<'_>> {
        self.edges
            .iter()
            .map(EdgeRef::from)
            .chain(self.edges.iter().map(EdgeRef::from).map(EdgeRef::reversed))
    }

    fn connections(&self) -> HashMap<&str, HashSet<&str>> {
        self.edges
            .iter()
            .fold(HashMap::<&str, HashSet<&str>>::new(), |mut map, conn| {
                map.entry(&conn.from).or_default().insert(&conn.to);
                map.entry(&conn.to).or_default().insert(&conn.from);
                map
            })
    }

    fn cliques(&self) -> impl Iterator<Item = Vec<&str>> {
        let connections = self.connections();
        let mut cliques: Vec<Vec<&str>> = Vec::new();
        for pc in self.nodes().sorted_unstable() {
            let conns = &connections[pc];
            for set in &mut cliques {
                if set.iter().copied().all(|other| conns.contains(other)) {
                    set.push(pc);
                }
            }
            cliques.push([pc].into());
        }
        cliques.into_iter()
    }
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        s.lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .filter_map(|line| line.split_once('-'))
            .map(|(a, b)| Edge {
                from: a.to_owned(),
                to: b.to_owned(),
            })
            .collect_vec()
            .pipe(|connections| Self { edges: connections })
            .pipe(Ok)
    }
}

#[derive(Debug)]
struct Edge {
    from: String,
    to: String,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct EdgeRef<'a> {
    from: &'a str,
    to: &'a str,
}

impl EdgeRef<'_> {
    const fn reversed(self) -> Self {
        Self {
            from: self.to,
            to: self.from,
        }
    }
}

impl<'a> From<&'a Edge> for EdgeRef<'a> {
    fn from(value: &'a Edge) -> Self {
        Self {
            from: &value.from,
            to: &value.to,
        }
    }
}

pub fn init_tracing() -> color_eyre::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

    let log_filter: EnvFilter = std::env::var(EnvFilter::DEFAULT_ENV)
        .map(String::leak)
        .map(|s| s as &str)
        .or_else(|err| match err {
            std::env::VarError::NotPresent => Ok("day_23=debug"),
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
