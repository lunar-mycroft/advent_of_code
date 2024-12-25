use itertools::Itertools;

use crate::{heights, is_key, is_lock, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> usize {
    let key_heights = puzzle
        .items
        .iter()
        .filter(|g| is_key(g))
        .map(heights)
        .collect_vec();
    let lock_heights = puzzle
        .items
        .iter()
        .filter(|g| is_lock(g))
        .map(heights)
        .collect_vec();
    lock_heights
        .into_iter()
        .cartesian_product(key_heights)
        .filter_map(|(lock, key)| lock.into_iter().zip(key).map(|(l, k)| l + k).max())
        .filter(|max_height| *max_height < 6)
        .count()
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 3)]
    #[case::example("part1.txt", 3249)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: usize) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
