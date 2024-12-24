use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{IntGraph, StringGraph};

#[must_use]
pub fn initial(puzzle: &StringGraph) -> String {
    let nodes: Vec<_> = puzzle
        .edges
        .iter()
        .flat_map(|conn| [conn.from.as_str(), conn.to.as_str()])
        .unique()
        .sorted_unstable()
        .collect_vec();
    let connected_to: HashMap<_, _> =
        puzzle
            .edges
            .iter()
            .fold(HashMap::<&str, HashSet<&str>>::new(), |mut map, conn| {
                map.entry(&conn.from).or_default().insert(&conn.to);
                map.entry(&conn.to).or_default().insert(&conn.from);
                map
            });
    let mut sets: Vec<HashSet<&str>> = Vec::new();
    for pc in nodes {
        let conns = &connected_to[pc];
        for set in &mut sets {
            if set.iter().copied().all(|other| conns.contains(other)) {
                set.insert(pc);
            }
        }
        sets.push([pc].into());
    }
    sets.into_iter()
        .max_by_key(HashSet::len)
        .expect("there to be sets")
        .into_iter()
        .sorted_unstable()
        .join(",")
}

#[must_use]
pub fn common_methods(puzzle: &StringGraph) -> String {
    puzzle
        .cliques()
        .max_by_key(Vec::len)
        .unwrap_or_default()
        .into_iter()
        .join(",")
}

#[must_use]
pub fn bron_kerbosh(puzzle: &StringGraph) -> String {
    fn bk_impl<'a>(
        g: &HashMap<&'a str, HashSet<&'a str>>,
        r: &mut HashSet<&'a str>,
        mut p: HashSet<&'a str>,
        mut x: HashSet<&'a str>,
        cliques: &mut Vec<HashSet<&'a str>>,
    ) {
        if p.is_empty() {
            if x.is_empty() {
                cliques.push(r.clone());
            }
            return;
        }
        while let Some(node) = p.iter().copied().next() {
            let neighbors = &g[node];
            r.insert(node);
            bk_impl(
                g,
                r,
                p.intersection(neighbors).copied().collect(),
                x.intersection(neighbors).copied().collect(),
                cliques,
            );
            r.remove(node);
            p.remove(node);
            x.insert(node);
        }
    }

    let mut cliques = Vec::new();
    let connections = puzzle.connections();
    bk_impl(
        &connections,
        &mut HashSet::new(),
        puzzle.nodes().collect(),
        HashSet::new(),
        &mut cliques,
    );
    cliques
        .iter()
        .max_by_key(|c| c.len())
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .sorted_unstable()
        .join(",")
}

#[must_use]
pub fn fx_hash(puzzle: &StringGraph) -> String {
    puzzle
        .cliques_fx()
        .max_by_key(Vec::len)
        .unwrap_or_default()
        .into_iter()
        .join(",")
}

pub fn int_graph(puzzle: &IntGraph) -> String {
    puzzle
        .cliques()
        .max_by_key(Vec::len)
        .unwrap_or_default()
        .into_iter()
        .map(|n| {
            let (a, b) = (
                u32::from(n / 26 + u16::from(b'a')),
                u32::from(n % 26 + u16::from(b'a')),
            );
            format!(
                "{}{}",
                char::from_u32(a).expect("known valid byte"),
                char::from_u32(b).expect("known valid byte")
            )
        })
        .join(",")
}

pub fn array(puzzle: &IntGraph) -> String {
    let (edges, nodes) = crate::array::parse(puzzle);
    crate::array::cliques(&edges, &nodes)
        .max_by_key(Vec::len)
        .unwrap_or_default()
        .into_iter()
        .map(|n| {
            let (a, b) = (
                u32::from(n / 26 + u16::from(b'a')),
                u32::from(n % 26 + u16::from(b'a')),
            );
            format!(
                "{}{}",
                char::from_u32(a).expect("known valid byte"),
                char::from_u32(b).expect("known valid byte")
            )
        })
        .join(",")
}

#[must_use]
pub fn array_inline(puzzle: &IntGraph) -> String {
    let (edges, nodes) = crate::array::parse(puzzle);
    let mut seen = [false; 676];
    let mut clique = Vec::with_capacity(16);
    let mut largest = Vec::with_capacity(16);
    for n1 in nodes.iter() {
        if seen[n1 as usize] {
            continue;
        }
        let neighbors = edges.get(n1);
        clique.clear();
        clique.push(n1);
        for n2 in neighbors.iter() {
            if clique.iter().all(|&c| edges.get(n2).contains(c)) {
                seen[n2 as usize] = true;
                clique.push(n2);
            }
        }
        if clique.len() > largest.len() {
            largest.clone_from(&clique);
        }
    }
    largest
        .into_iter()
        .map(|n| {
            let (a, b) = (
                u32::from(n / 26 + u16::from(b'a')),
                u32::from(n % 26 + u16::from(b'a')),
            );
            format!(
                "{}{}",
                char::from_u32(a).expect("known valid byte"),
                char::from_u32(b).expect("known valid byte")
            )
        })
        .join(",")
}

pub fn array_preparsed((edges, nodes): &(crate::array::EdgeMap, crate::array::NodeSet)) -> String {
    crate::array::cliques(edges, nodes)
        .max_by_key(Vec::len)
        .unwrap_or_default()
        .into_iter()
        .map(|n| {
            let (a, b) = (
                u32::from(n / 26 + u16::from(b'a')),
                u32::from(n % 26 + u16::from(b'a')),
            );
            format!(
                "{}{}",
                char::from_u32(a).expect("known valid byte"),
                char::from_u32(b).expect("known valid byte")
            )
        })
        .join(",")
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;
    use tap::prelude::*;

    use super::*;

    #[rstest]
    #[case::example("example.txt", "co,de,ka,ta")]
    #[case::example("part2.txt", "cl,ei,fd,hc,ib,kq,kv,ky,rv,vf,wk,yx,zf")]
    fn finds_solution(#[case] input_path: &str, #[case] expected: &str) -> Result<()> {
        let string: StringGraph = common::read_input!(input_path).parse()?;
        let int: IntGraph = common::read_input!(input_path).parse()?;
        assert_eq!(initial(&string), expected);
        assert_eq!(common_methods(&string), expected);
        assert_eq!(fx_hash(&string), expected);
        assert_eq!(int_graph(&int), expected);
        assert_eq!(array(&int), expected);
        assert_eq!(array_inline(&int), expected);
        assert_eq!(
            crate::array::parse(&int).pipe_ref(array_preparsed),
            expected
        );
        assert_eq!(bron_kerbosh(&string), expected);
        Ok(())
    }
}
