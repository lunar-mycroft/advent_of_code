use itertools::Itertools;
use tap::prelude::*;

use crate::{Operator, Puzzle};

#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> Result<u64, std::num::ParseIntError> {
    let mut rows = puzzle
        .numbers
        .trim()
        .lines()
        .map(str::split_whitespace)
        .collect_vec();
    puzzle
        .operations
        .into_iter()
        .map(|op| {
            rows.iter_mut()
                .filter_map(Iterator::next)
                .map(str::parse::<u64>)
                .pipe(|it| op.solve(it))
        })
        .pipe(|it| Operator::Add.solve(it))
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 4_277_556)]
    #[case::puzzle("part1.txt", 6_957_525_317_641)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input = common::read_input!(input_path).parse()?;
        let output = process(input)?;
        assert_eq!(output, expected);
        Ok(())
    }
}
