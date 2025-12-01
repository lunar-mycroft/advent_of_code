use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value, clippy::cast_sign_loss)]
pub fn process(puzzle: Puzzle) -> u64 {
    let mut position: i16 = 50;
    puzzle.rotations.into_iter().fold(0, |acc, rotation| {
        let (delta, new) = {
            let (full_rotations, extra) = ((rotation / 100).abs(), rotation % 100);
            match (position + extra, position) {
                (0, _) => (full_rotations as u64 + 1, 0),
                (n, 0) => (full_rotations as u64, n.rem_euclid(100)),
                (n @ 0..=99, _) => (full_rotations as u64, n),
                (n, _) => ((full_rotations as u64) + 1, n.rem_euclid(100)),
            }
        };
        position = new;
        acc + delta
    })
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
        // not 7175 or 6967
        assert_eq!(output, expected);
        Ok(())
    }
}
