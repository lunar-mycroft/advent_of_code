use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> u64 {
    puzzle.banks.iter().map(|bank| joltage(&bank.0)).sum()
}

fn joltage(mut batteries: &[u8]) -> u64 {
    let mut res = 0;
    for len in (1usize..=12).rev() {
        let (start, n) = digit(batteries, len).expect("");
        res = res * 10 + u64::from(n);
        batteries = &batteries[start + 1..];
    }
    res
}

fn digit(batteries: &[u8], len: usize) -> Option<(usize, u8)> {
    batteries[..=(batteries.len() - len)]
        .iter()
        .copied()
        .enumerate()
        .rev()
        .max_by_key(|(_, digit)| *digit)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 3_121_910_778_619)]
    #[case::part2("part2.txt", 169_347_417_057_382)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }

    #[rstest]
    #[case(&[9,8,7,6,5,4,3,2,1,1,1,1,1,1,1], 987_654_321_111)]
    #[case(&[8,1,1,1,1,1,1,1,1,1,1,1,1,1,9], 811_111_111_119)]
    #[case(&[2,3,4,2,3,4,2,3,4,2,3,4,2,7,8], 434_234_234_278)]
    #[case(&[8,1,8,1,8,1,9,1,1,1,1,2,1,1,1], 888_911_112_111)]
    fn find_joltage(#[case] batteries: &[u8], #[case] expected: u64) {
        assert_eq!(joltage(batteries), expected);
    }
}
