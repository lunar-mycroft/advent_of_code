use itertools::Itertools as _;
use tap::prelude::*;

use crate::{Dsu, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(Puzzle { boxes }: Puzzle) -> i64 {
    todo!();
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 25_272)]
    #[case::puzzle("part2.txt", 0)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: i64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
