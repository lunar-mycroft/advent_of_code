use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};
use tap::prelude::*;

use crate::Puzzle;

// #[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> Result<u64> {
    puzzle
        .ranges
        .iter()
        .filter(|(start, end)| !(start.is_empty() || end.is_empty()))
        .map(|(start, end)| {
            repeats(start, end)
                .wrap_err_with(|| format!("{start:?}, {end:?}"))?
                .sum::<u64>()
                .pipe(Ok::<_, color_eyre::Report>)
        })
        .try_fold(0, |sum, res| Ok(sum + res?))
}

fn repeats(start: &str, end: &str) -> Result<impl Iterator<Item = u64>> {
    let end: u64 = end.parse()?;
    let half = 10u64.pow((start.len().try_conv::<u32>()? / 2).max(1) - 1);
    let start: u64 = start.parse().wrap_err("start")?;
    (half..)
        .map(|n| 10u64.pow(n.checked_ilog10().unwrap_or(0) + 1) * n + n)
        .skip_while(move |&n| n < start)
        .take_while(move |&n| n <= end)
        .pipe(Ok)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 1_227_775_554)]
    #[case::part1("part1.txt", 30_599_400_849)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        // not 24_328_657_108
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input)?;
        assert_eq!(output, expected);
        Ok(())
    }

    #[rstest]
    #[case("11", "22", vec![11, 22])]
    #[case("95", "115", vec![99])]
    #[case("998", "1012", vec![1010])]
    #[case("1188511880", "1188511890", vec![1_188_511_885])]
    #[case("222220", "222224", vec![222_222])]
    #[case("1698522", "1698528", vec![])]
    #[case("446443", "446449", vec![446_446])]
    #[case("38593856", "38593862", vec![38_593_859])]
    #[case("3", "17", vec![11])]
    fn repeat_works(
        #[case] start: &str,
        #[case] end: &str,
        #[case] expected: Vec<u64>,
    ) -> Result<()> {
        assert_eq!(10u64.checked_ilog10().unwrap_or(0) + 1, 2);
        let v: Vec<_> = repeats(start, end)?.collect();
        assert_eq!(v, expected);
        Ok(())
    }
}
