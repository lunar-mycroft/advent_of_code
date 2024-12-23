use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::Puzzle;

#[must_use]
pub fn initial(puzzle: &Puzzle) -> String {
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
pub fn common_methods(puzzle: &Puzzle) -> String {
    puzzle
        .cliques()
        .max_by_key(Vec::len)
        .unwrap_or_default()
        .into_iter()
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
        assert_eq!(initial(&input), expected);
        assert_eq!(common_methods(&input), expected);
        Ok(())
    }
}
