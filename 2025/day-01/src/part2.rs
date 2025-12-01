use core::panic;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value, clippy::cast_sign_loss)]
pub fn process(puzzle: Puzzle) -> u64 {
    let mut position: i16 = 50;
    puzzle
        .rotations
        .into_iter()
        .map(|rotation| {
            let (full_rotations, extra) = ((rotation / 100).abs(), rotation % 100);
            let new = (position + extra).rem_euclid(100);
            let delta = match (position + extra, position) {
                (0, _) => 1,
                (_, 0) | (0..=99, _) => 0,
                (_, _) => 1,
            };
            position = new;
            delta + (full_rotations as u64)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 6)]
    #[case::example("part2.txt", 6932)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
