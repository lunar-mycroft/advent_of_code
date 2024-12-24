use crate::{Puzzle, Wire};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> u64 {
    (0..=45)
        .map(Wire::Z)
        .map(|wire| puzzle.eval(wire))
        .rev()
        .fold(0, |mut res, bit| {
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
