use itertools::Itertools;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> u64 {
    let you = puzzle.you.expect("To find you");
    let order = puzzle.topological_order();
    let ((i, start), (j, end)) = order
        .iter()
        .copied()
        .enumerate()
        .filter(|&(_, node)| node == you || node == puzzle.out)
        .collect_tuple()
        .expect("Missing you our out");
    debug_assert_eq!(start, you);
    debug_assert_eq!(end, puzzle.out);
    puzzle.num_paths(you, puzzle.out, &order[i..j])
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 5)]
    #[case::puzzle("input.txt", 753)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
