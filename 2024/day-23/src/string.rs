use std::collections::{HashMap, HashSet};

use color_eyre::Result;
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use tap::prelude::*;

#[derive(Debug)]
pub struct Graph {
    pub(crate) edges: Vec<Edge>,
}

#[derive(Debug)]
pub(crate) struct Edge {
    pub(crate) from: String,
    pub(crate) to: String,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub(crate) struct EdgeRef<'a> {
    pub(crate) from: &'a str,
    pub(crate) to: &'a str,
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
impl Graph {
    pub(crate) fn nodes(&self) -> impl Iterator<Item = &str> {
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

    pub(crate) fn all_edges(&self) -> impl Iterator<Item = EdgeRef<'_>> {
        self.edges
            .iter()
            .map(EdgeRef::from)
            .chain(self.edges.iter().map(EdgeRef::from).map(EdgeRef::reversed))
    }

    pub(crate) fn connections(&self) -> HashMap<&str, HashSet<&str>> {
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

    pub(crate) fn cliques(&self) -> impl Iterator<Item = Vec<&str>> {
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

    pub(crate) fn cliques_fx(&self) -> impl Iterator<Item = Vec<&str>> {
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

impl std::str::FromStr for Graph {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
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
