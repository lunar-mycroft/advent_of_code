use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> u64 {
    let order = puzzle.topological_order();
    let (dac, fft) = (
        puzzle.dac.expect("to find dac"),
        puzzle.fft.expect("to find fft"),
    );

    puzzle.num_paths(puzzle.svr.expect("To find svr"), dac, &order)
        * puzzle.num_paths(dac, fft, &order)
        * puzzle.num_paths(fft, puzzle.out, &order)
        + puzzle.num_paths(puzzle.svr.expect("to find svr"), fft, &order)
            * puzzle.num_paths(fft, dac, &order)
            * puzzle.num_paths(dac, puzzle.out, &order)
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
