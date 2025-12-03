use tap::prelude::*;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> u32 {
    puzzle
        .banks
        .iter()
        .map(|bank| joltage(&bank.0))
        .map(u32::from)
        .sum()
}

fn joltage(batteries: &[u8]) -> u8 {
    (0..(batteries.len() - 1))
        .map(|start| {
            (
                batteries[start],
                batteries[start + 1..]
                    .iter()
                    .copied()
                    .max()
                    .expect("slices to be at least two long"),
            )
        })
        .map(|(first, second)| first * 10 + second)
        .max()
        .expect("slices to be at least two long")
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 357)]
    #[case::example("part1.txt", 0)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u32) -> Result<()> {
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
    fn find_joltage(#[case] batteries: &[u8], #[case] expected: u8) {
        assert_eq!(joltage(batteries), expected);
    }
}
