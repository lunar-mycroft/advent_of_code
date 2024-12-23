use std::collections::{HashMap, HashSet};

use color_eyre::eyre::bail;
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use tap::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct StringGraph {
    edges: Vec<StringEdge>,
}

#[derive(Debug)]
pub struct IntGraph {
    edges: Vec<IntEdge>,
}

impl StringGraph {
    fn nodes(&self) -> impl Iterator<Item = &str> {
        self.edges
            .iter()
            .flat_map(|conn| [conn.from.as_str(), conn.to.as_str()])
            .unique()
    }

    fn nodes_fx(&self) -> impl Iterator<Item = &str> {
        let mut set = FxHashSet::default();
        for edge in &self.edges {
            set.insert(edge.from.as_str());
            set.insert(edge.to.as_str());
        }
        set.into_iter()
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

    fn connections_fx(&self) -> FxHashMap<&str, FxHashSet<&str>> {
        self.edges.iter().fold(
            FxHashMap::<&str, FxHashSet<&str>>::default(),
            |mut map, conn| {
                map.entry(&conn.from).or_default().insert(&conn.to);
                map.entry(&conn.to).or_default().insert(&conn.from);
                map
            },
        )
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

    fn cliques_fx(&self) -> impl Iterator<Item = Vec<&str>> {
        let connections = self.connections_fx();
        let mut cliques: Vec<Vec<&str>> = Vec::new();
        for pc in self.nodes_fx().sorted_unstable() {
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

impl IntGraph {
    fn nodes(&self) -> impl Iterator<Item = u16> {
        let mut set = FxHashSet::default();
        for edge in &self.edges {
            set.insert(edge.from);
            set.insert(edge.to);
        }
        set.into_iter()
    }

    fn connections(&self) -> FxHashMap<u16, FxHashSet<u16>> {
        self.edges.iter().fold(
            FxHashMap::<u16, FxHashSet<u16>>::default(),
            |mut map, conn| {
                map.entry(conn.from).or_default().insert(conn.to);
                map.entry(conn.to).or_default().insert(conn.from);
                map
            },
        )
    }

    fn cliques(&self) -> impl Iterator<Item = Vec<u16>> + '_ {
        let connections = self.connections();
        let mut cliques: Vec<Vec<u16>> = Vec::new();
        for pc in self.nodes().sorted_unstable() {
            let conns = &connections[&pc];
            for set in &mut cliques {
                if set.iter().copied().all(|other| conns.contains(&other)) {
                    set.push(pc);
                }
            }
            cliques.push([pc].into());
        }
        cliques.into_iter()
    }
}

impl std::str::FromStr for StringGraph {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        s.lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .filter_map(|line| line.split_once('-'))
            .map(|(a, b)| StringEdge {
                from: a.to_owned(),
                to: b.to_owned(),
            })
            .collect_vec()
            .pipe(|connections| Self { edges: connections })
            .pipe(Ok)
    }
}

impl std::str::FromStr for IntGraph {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        fn to_u16(s: &str) -> color_eyre::Result<u16> {
            match s.as_bytes() {
                [a, b] if a.is_ascii_lowercase() && b.is_ascii_lowercase() => {
                    Ok(u16::from(a - b'a') * 26 + u16::from(b - b'a'))
                }
                [_, _] => bail!("Invalid character in {s:?}"),
                _ => bail!("Incorrect length of {s:?}"),
            }
        }
        s.lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .filter_map(|line| line.split_once('-'))
            .map(|(a, b)| {
                IntEdge {
                    from: to_u16(a)?,
                    to: to_u16(b)?,
                }
                .pipe(Ok::<_, color_eyre::Report>)
            })
            .try_collect::<_, Vec<_>, _>()?
            .pipe(|connections| Self { edges: connections })
            .pipe(Ok)
    }
}

#[derive(Debug)]
struct StringEdge {
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

impl<'a> From<&'a StringEdge> for EdgeRef<'a> {
    fn from(value: &'a StringEdge) -> Self {
        Self {
            from: &value.from,
            to: &value.to,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct IntEdge {
    from: u16,
    to: u16,
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
