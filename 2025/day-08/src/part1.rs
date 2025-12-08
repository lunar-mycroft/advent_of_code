use std::collections::{HashMap, HashSet};

use glam::I64Vec3 as IVec3;
use itertools::Itertools;

use crate::{Dsu, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(Puzzle { boxes }: Puzzle, n: usize) -> usize {
    let by_distance = {
        let mut res = boxes
            .iter()
            .tuple_combinations::<(_, _)>()
            .map(|(a, b)| (*a, *b, a.distance_squared(*b)))
            .collect_vec();
        res.sort_unstable_by_key(|(_, _, r)| *r);
        res.truncate(n);
        res
    };
    let mut circuits: HashMap<IVec3, HashSet<IVec3>> = HashMap::new();
    let mut dsu = Dsu::default();
    for (u, v, _) in by_distance {
        dsu.unite(u, v);
    }
    for junction in boxes {
        circuits
            .entry(dsu.parent(junction))
            .or_default()
            .insert(junction);
    }
    {
        let mut v = circuits.values().map(HashSet::len).collect_vec();
        v.sort_unstable();
        v.reverse();
        v[..3].iter().copied().product()
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 10, 40)]
    #[case::puzzle("part1.txt", 1000, 62_186)]
    fn finds_solution(
        #[case] input_path: &str,
        #[case] n: usize,
        #[case] expected: usize,
    ) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input, n);
        assert_eq!(output, expected);
        Ok(())
    }
}
