use crate::{next_num, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> u64 {
    puzzle
        .numbers
        .iter()
        .copied()
        .map(|seed| nth_num(seed, 2000))
        .map(u64::from)
        .sum()
}

fn nth_num(mut seed: u32, n: u16) -> u32 {
    for _ in 0..n {
        seed = next_num(seed);
    }
    seed
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(1, 8_685_429)]
    fn test_nth(#[case] seed: u32, #[case] value: u32) {
        assert_eq!(nth_num(seed, 2000), value);
    }

    #[rstest]
    #[case::actual("part1.txt", 13_185_239_446)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
