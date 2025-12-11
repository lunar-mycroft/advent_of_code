use crate::{Puzzle, OUT};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> u64 {
    const DAC: [u8; 3] = [b'd', b'a', b'c'];
    const FFT: [u8; 3] = [b'f', b'f', b't'];
    const SVR: [u8; 3] = [b's', b'v', b'r'];

    let order = puzzle.topological_order();

    puzzle.num_paths(SVR, DAC, &order)
        * puzzle.num_paths(DAC, FFT, &order)
        * puzzle.num_paths(FFT, OUT, &order)
        + puzzle.num_paths(SVR, FFT, &order)
            * puzzle.num_paths(FFT, DAC, &order)
            * puzzle.num_paths(DAC, OUT, &order)
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
