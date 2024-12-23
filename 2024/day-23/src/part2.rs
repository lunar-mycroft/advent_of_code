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

#[must_use]
pub fn bron_kerbosh(puzzle: &Puzzle) -> String {
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
pub fn fx_hash(puzzle: &Puzzle) -> String {
    puzzle
        .cliques_fx()
        .max_by_key(Vec::len)
        .unwrap_or_default()
        .into_iter()
        .join(",")
}

// #[must_use]
// pub fn counter(puzzle: &Puzzle) -> String {
//     let counter = puzzle.edges.iter().fold(HashMap::new(), |mut map, edge| {
//         *map.entry(edge.to.as_str()).or_insert(0usize) += 1;
//         map
//     });
//     let max_count = *counter.values().max().expect("there to be values");
//     counter
//         .into_iter()
//         .filter(|(_, n)| *n == max_count)
//         .map(|(s, _)| s)
//         .sorted_unstable()
//         .join(",")
// }

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
        assert_eq!(fx_hash(&input), expected);
        assert_eq!(bron_kerbosh(&input), expected);
        // assert_eq!(counter(&input), expected);
        Ok(())
    }
}
