use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};
use itertools::Itertools;
use tap::prelude::*;

use crate::Puzzle;

#[must_use]
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

fn repeats_n(len: u32, start_n: u64, end_n: u64, repetitions: u32) -> impl Iterator<Item = u64> {
    let half = 10u64.pow((len / repetitions).max(1) - 1);
    (half..)
        .map(move |n| {
            (0..repetitions)
                .map(|p| 10u64.pow((n.checked_ilog10().unwrap_or(0) + 1) * p) * n)
                .sum::<u64>()
        })
        .skip_while(move |&n| n < start_n)
        .take_while(move |&n| n <= end_n)
}

fn repeats(start: &str, end: &str) -> Result<impl Iterator<Item = u64>> {
    let start_n: u64 = start.parse()?;
    let end_n: u64 = end.parse()?;
    let len: u32 = start.len().try_conv()?;
    (2..=end.len().try_conv::<u32>()?.max(len))
        .flat_map(move |n| repeats_n(len, start_n, end_n, n))
        .unique()
        .pipe(Ok)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 4_174_379_265)]
    #[case::example("part2.txt", 0)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input)?;
        assert_eq!(output, expected);
        Ok(())
    }

    #[rstest]
    #[case::a("11", "22", vec![11, 22])]
    #[case::b("95", "115", vec![99, 111])]
    #[case::c("998", "1012", vec![999, 1010])]
    #[case::d("1188511880", "1188511890", vec![1_188_511_885])]
    #[case::e("222220", "222224", vec![222_222])]
    #[case::f("1698522", "1698528", vec![])]
    #[case::g("446443", "446449", vec![446_446])]
    #[case::h("38593856", "38593862", vec![38_593_859])]
    #[case::i("3", "17", vec![11])]
    fn repeat_works(
        #[case] start: &str,
        #[case] end: &str,
        #[case] expected: Vec<u64>,
    ) -> Result<()> {
        let mut v: Vec<_> = repeats(start, end)?.collect();
        v.sort_unstable();
        assert_eq!(v, expected);
        Ok(())
    }
}
