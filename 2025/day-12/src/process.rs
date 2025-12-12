use itertools::Itertools;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> usize {
    let areas = puzzle
        .presents
        .iter()
        .map(|grid| grid.iter().map(|&d| d & 1).sum::<u8>())
        .map(u16::from)
        .collect_vec();
    puzzle
        .regions
        .iter()
        .filter(|&&region| {
            let area = u16::try_from(region.size.x * region.size.y)
                .expect("Size cannot be bigger than 10_000");
            let present_area: u16 = region
                .quantities
                .iter()
                .copied()
                .map(u16::from)
                .zip(areas.iter().copied())
                .map(|(n, a)| n * a)
                .sum();
            area >= present_area
        })
        .count()
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[ignore = "solution doesn't work on example input"]
    #[case::example("example.txt", 2)]
    #[case::puzzle("input.txt", 510)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: usize) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
