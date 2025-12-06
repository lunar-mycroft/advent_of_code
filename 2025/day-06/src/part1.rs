use tap::prelude::*;

use crate::{Problem, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> u64 {
    puzzle
        .problems
        .iter()
        .map(|problem| match problem {
            Problem::Add(items) => items.iter().copied().sum::<u64>(),
            Problem::Multiply(items) => items.iter().copied().product::<u64>(),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 4_277_556)]
    #[case::example("part1.txt", 0)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
