use crate::{joltage, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> u64 {
    puzzle
        .banks
        .iter()
        .map(|bank| joltage(&bank.0, 2).expect("bank.0.len() to be >= 2"))
        .sum()
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 357)]
    #[case::part1("part1.txt", 17_109)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }

    #[rstest]
    #[case(&[9,8,7,6,5,4,3,2,1,1,1,1,1,1,1], 98)]
    #[case(&[8,1,1,1,1,1,1,1,1,1,1,1,1,1,9], 89)]
    #[case(&[2,3,4,2,3,4,2,3,4,2,3,4,2,7,8], 78)]
    #[case(&[8,1,8,1,8,1,9,1,1,1,1,2,1,1,1], 92)]
    #[case(&[9,8,9,8,9,8,9,8,9,8,9,8,9,8,9,8,9,8], 99)]
    fn find_joltage(#[case] batteries: &[u8], #[case] expected: u64) {
        assert_eq!(joltage(batteries, 2), Some(expected));
    }
}
