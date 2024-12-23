use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{EdgeRef, Puzzle};

#[must_use]
pub fn initial(puzzle: &Puzzle) -> usize {
    let connected_to: HashMap<_, _> =
        puzzle
            .edges
            .iter()
            .fold(HashMap::<&str, HashSet<&str>>::new(), |mut map, conn| {
                map.entry(&conn.from).or_default().insert(&conn.to);
                map.entry(&conn.to).or_default().insert(&conn.from);
                map
            });
    let n = connected_to
        .iter()
        .flat_map(|(key, connections)| {
            connections
                .iter()
                .combinations(2)
                .filter(|pair| connected_to[pair[0]].contains(pair[1]))
                .map(|pair| {
                    let mut arr = [*key, *pair[0], *pair[1]];
                    arr.sort_unstable();
                    arr
                })
                .filter(|trip| trip.iter().copied().any(|pc| pc.starts_with('t')))
        })
        .collect::<HashSet<_>>();
    n.len()
}

#[must_use]
pub fn common_methods(puzzle: &Puzzle) -> usize {
    let connected_to = puzzle.connections();
    connected_to
        .iter()
        .flat_map(|(key, connections)| {
            connections
                .iter()
                .combinations(2)
                .filter(|pair| connected_to[pair[0]].contains(pair[1]))
                .map(|pair| {
                    let mut arr = [*key, *pair[0], *pair[1]];
                    arr.sort_unstable();
                    arr
                })
                .filter(|trip| trip.iter().copied().any(|pc| pc.starts_with('t')))
        })
        .unique()
        .count()
}

#[must_use]
pub fn edge_set(puzzle: &Puzzle) -> usize {
    let edges: HashSet<_> = puzzle.all_edges().collect();
    puzzle
        .connections()
        .iter()
        .flat_map(|(key, connections)| {
            connections
                .iter()
                .combinations(2)
                .map(|pair| EdgeRef {
                    from: pair[0],
                    to: pair[1],
                })
                .filter(|edge| edges.contains(edge))
                .map(|pair| {
                    let mut arr = [*key, pair.from, pair.to];
                    arr.sort_unstable();
                    arr
                })
                .filter(|trip| trip.iter().copied().any(|pc| pc.starts_with('t')))
        })
        .unique()
        .count()
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 7)]
    #[case::example("part1.txt", 1358)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: usize) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        assert_eq!(initial(&input), expected);
        assert_eq!(common_methods(&input), expected);
        assert_eq!(edge_set(&input), expected);
        Ok(())
    }
}
