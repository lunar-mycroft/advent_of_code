use color_eyre::eyre::{bail, Report, Result};
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools as _;
use tap::prelude::*;

#[derive(Debug)]
pub struct Graph {
    edges: Vec<Edge>,
}

impl Graph {
    pub(crate) fn all_edges(&self) -> impl Iterator<Item = Edge> + '_ {
        self.edges
            .iter()
            .copied()
            .chain(self.edges.iter().copied().map(Edge::reversed))
    }

    fn nodes(&self) -> impl Iterator<Item = u16> {
        let mut set = FxHashSet::default();
        for edge in &self.edges {
            set.insert(edge.from);
            set.insert(edge.to);
        }
        set.into_iter()
    }

    pub(crate) fn connections(&self) -> FxHashMap<u16, FxHashSet<u16>> {
        self.edges.iter().fold(
            FxHashMap::<u16, FxHashSet<u16>>::default(),
            |mut map, conn| {
                map.entry(conn.from).or_default().insert(conn.to);
                map.entry(conn.to).or_default().insert(conn.from);
                map
            },
        )
    }

    pub(crate) fn cliques(&self) -> impl Iterator<Item = Vec<u16>> + '_ {
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

impl std::str::FromStr for Graph {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        fn to_u16(s: &str) -> Result<u16> {
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
                Edge {
                    from: to_u16(a)?,
                    to: to_u16(b)?,
                }
                .pipe(Ok::<_, Report>)
            })
            .try_collect::<_, Vec<_>, _>()?
            .pipe(|connections| Self { edges: connections })
            .pipe(Ok)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Edge {
    pub(crate) from: u16,
    pub(crate) to: u16,
}

impl Edge {
    const fn reversed(self) -> Self {
        Self {
            from: self.to,
            to: self.from,
        }
    }
}
