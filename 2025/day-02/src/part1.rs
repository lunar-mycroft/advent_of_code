use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> u64 {
    puzzle
        .ranges
        .iter()
        .flat_map(|r| r.repeat_n(2))
        .sum::<u64>()
}

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process_specialized(puzzle: Puzzle) -> u64 {
    puzzle.ranges.iter().copied().flat_map(repeats).sum::<u64>()
}

fn repeats(range: crate::Range) -> impl Iterator<Item = u64> {
    let half = 10u64.pow((range.start_len / 2).max(1) - 1);
    (half..)
        .map(|n| 10u64.pow(n.checked_ilog10().unwrap_or(0) + 1) * n + n)
        .skip_while(move |&n| n < range.start)
        .take_while(move |&n| n <= range.end)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;
    use tap::prelude::*;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 1_227_775_554)]
    #[case::part1("part1.txt", 30_599_400_849)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = input.clone().pipe(process);
        assert_eq!(output, expected);
        assert_eq!(process_specialized(input), expected);
        Ok(())
    }
}
