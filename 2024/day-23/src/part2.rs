use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: &Puzzle) -> String {
    let nodes: Vec<_> = puzzle
        .connections
        .iter()
        .flat_map(|conn| [conn.from.as_str(), conn.to.as_str()])
        .unique()
        .sorted_unstable()
        .collect_vec();
    let connectioned_to: HashMap<_, _> =
        puzzle
            .connections
            .iter()
            .fold(HashMap::<&str, HashSet<&str>>::new(), |mut map, conn| {
                map.entry(&conn.from).or_default().insert(&conn.to);
                map.entry(&conn.to).or_default().insert(&conn.from);
                map
            });
    let mut sets: Vec<HashSet<&str>> = Vec::new();
    for pc in nodes {
        let conns = &connectioned_to[pc];
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

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", "co,de,ka,ta")]
    #[case::example("part2.txt", "cl,ei,fd,hc,ib,kq,kv,ky,rv,vf,wk,yx,zf")]
    fn finds_solution(#[case] input_path: &str, #[case] expected: &str) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(&input);
        assert_eq!(output, expected);
        Ok(())
    }
}
