use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};
use tap::prelude::*;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> Result<u64> {
    todo!("day_02 part2")
}

fn repeats(start: &str, end: &str) -> Result<impl Iterator<Item = u64>> {
    let end: u64 = end.parse()?;
    let half = 10u64.pow((start.len().try_conv::<u32>()? / 2).max(1) - 1);
    let start: u64 = start.parse().wrap_err("start")?;
    (half..)
        .flat_map(|n| {
            let digits = n.checked_ilog10().unwrap_or(0) + 1;
            (2u32..).map(move |p| {
                (1..=p)
                    .map(|q| 10u64.strict_pow(q * digits) * n)
                    .sum::<u64>()
            })
        })
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
    #[case::example("example.txt", 4_174_379_265)]
    #[case::example("part2.txt", 0)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input)?;
        assert_eq!(output, expected);
        Ok(())
    }

    #[rstest]
    #[case("11", "22", vec![11, 22])]
    #[case("95", "115", vec![99, 111])]
    #[case("998", "1012", vec![999, 1010])]
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
        let mut v: Vec<_> = repeats(start, end)?.collect();
        v.sort_unstable();
        assert_eq!(v, expected);
        Ok(())
    }
}
