use std::ops::RangeInclusive;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(mut puzzle: Puzzle) -> u64 {
    puzzle.ranges.sort_by_key(|r| *r.start());
    let len = puzzle.ranges.len();
    let ranges = puzzle.ranges.into_iter().fold(
        Vec::<RangeInclusive<_>>::with_capacity(len),
        |mut v, curr| {
            match v.last_mut() {
                Some(last) if curr.start() <= last.end() => {
                    *last = *last.start()..=*(last.end().max(curr.end()));
                }
                Some(_) | None => v.push(curr),
            }
            v
        },
    );
    ranges
        .into_iter()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 14)]
    #[case::puzzle("part2.txt", 347_468_726_696_961)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
