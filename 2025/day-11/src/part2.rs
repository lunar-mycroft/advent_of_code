use itertools::Itertools;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> u64 {
    let (dac, fft, svr) = (
        puzzle.dac.expect("to find dac"),
        puzzle.fft.expect("to find fft"),
        puzzle.svr.expect("To find svr"),
    );
    debug_assert_ne!(dac, fft);

    let order = puzzle.topological_order();
    let segments = order
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, node)| [dac, fft, svr, puzzle.out].contains(node))
        .collect_array::<4>()
        .expect("Missing at least one of the required nodes");
    debug_assert_eq!(segments[0].1, svr);
    debug_assert_eq!(segments[3].1, puzzle.out);
    debug_assert_ne!(segments[1].1, segments[2].1);
    segments
        .into_iter()
        .tuple_windows()
        .map(|((i, from), (j, to))| puzzle.num_paths(from, to, &order[i..j]))
        .product()
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example2.txt", 2)]
    #[case::puzzle("input.txt", 450_854_305_019_580)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert!(output > 184 || expected == 2);
        assert_eq!(output, expected);
        Ok(())
    }
}
