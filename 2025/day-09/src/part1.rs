use itertools::Itertools;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(Puzzle { tiles }: Puzzle) -> u64 {
    tiles
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| crate::area(a, b))
        .max()
        .expect("Knonw non-empty")
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 50)]
    #[case::puzzle("input.txt", 4_776_100_539)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
