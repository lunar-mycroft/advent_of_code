use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: &Puzzle) -> usize {
    let connectioned_to: HashMap<_, _> =
        puzzle
            .connections
            .iter()
            .fold(HashMap::<&str, HashSet<&str>>::new(), |mut map, conn| {
                map.entry(&conn.from).or_default().insert(&conn.to);
                map.entry(&conn.to).or_default().insert(&conn.from);
                map
            });
    let n = connectioned_to
        .iter()
        .flat_map(|(key, connections)| {
            connections
                .iter()
                .combinations(2)
                .filter(|pair| connectioned_to[pair[0]].contains(pair[1]))
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
        let output = process(&input);
        assert_ne!(output, 2_264);
        assert_eq!(output, expected);
        Ok(())
    }
}
