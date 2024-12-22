use crate::{Puzzle, Rng};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> u64 {
    puzzle
        .numbers
        .iter()
        .copied()
        .filter_map(nth::<2000>)
        .map(u64::from)
        .sum()
}

#[inline]
fn nth<const N: usize>(seed: u32) -> Option<u32> {
    Rng(seed).nth(N)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(1, 8_685_429)]
    fn test_nth(#[case] seed: u32, #[case] value: u32) {
        assert_eq!(nth::<2000>(seed), Some(value));
    }

    #[rstest]
    #[case::actual("part1.txt", 13_185_239_446)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
