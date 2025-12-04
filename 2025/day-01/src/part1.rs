use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> u64 {
    let mut position: i16 = 50;
    puzzle
        .rotations
        .into_iter()
        .map(|rotation| {
            position = (position + rotation).rem_euclid(100);
            u64::from(position == 0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 3)]
    #[case::puzzle("part1.txt", 1172)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
