use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use tap::prelude::*;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(mut puzzle: Puzzle) -> u64 {
    let mut n = 0;
    while puzzle.operations.keys().any(|id| id.starts_with('z')) {
        n += 1;
        let mut new_ops = FxHashMap::default();
        for (dest, (lhs, op, rhs)) in puzzle.operations {
            let (Some(lhs), Some(rhs)) = (
                puzzle.state.get(&lhs).copied(),
                puzzle.state.get(&rhs).copied(),
            ) else {
                new_ops.insert(dest, (lhs, op, rhs));
                continue;
            };
            let out = match op.as_str() {
                "OR" => lhs || rhs,
                "XOR" => lhs != rhs,
                "AND" => lhs && rhs,
                _ => unreachable!(),
            };
            puzzle.state.insert(dest, out);
        }
        puzzle.operations = new_ops;
    }
    puzzle
        .state
        .into_iter()
        .filter(|(s, _)| s.starts_with('z'))
        .sorted_by_key(|(s, _)| s.clone())
        .rev()
        .fold(0, |mut res, (_, bit)| {
            res <<= 1;
            res |= u64::from(bit);
            res
        })
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[ignore]
    #[case::example("example.txt", 2024)]
    #[case::example("part1.txt", 55_920_211_035_878)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
