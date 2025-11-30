use tap::prelude::*;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> String {
    todo!("{{crate_name}} part1")
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", "{{crate_name}} part1")]
    #[case::example("part1.txt", "{{crate_name}} part1")]
    fn finds_solution(#[case] input_path: &str, #[case] expected: &str) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
