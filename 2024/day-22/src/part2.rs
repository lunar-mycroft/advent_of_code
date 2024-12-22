use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use rayon::prelude::*;

use crate::{next_num, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> u64 {
    let all_sequences = puzzle
        .numbers
        .par_iter()
        .flat_map_iter(|n| sequences(*n))
        .collect::<HashSet<_>>();
    let all_buy_prices = puzzle.numbers.into_iter().map(buy_prices).collect_vec();
    all_sequences
        .into_par_iter()
        .map(|seq| {
            all_buy_prices
                .iter()
                .filter_map(|prices| {
                    prices
                        .get(&seq)
                        .copied()
                        .map(u64::try_from)
                        .transpose()
                        .expect("")
                })
                .sum()
        })
        .max()
        .unwrap_or(0)
}

fn buy_prices(seed: u64) -> HashMap<(i8, i8, i8, i8), i8> {
    rng(seed).take(2001).map(price).tuple_windows().fold(
        HashMap::new(),
        |mut map, (a, b, c, d, e)| {
            let seq = (b - a, c - b, d - c, e - d);
            map.entry(seq).or_insert(e);
            map
        },
    )
}

fn sequences(seed: u64) -> impl Iterator<Item = (i8, i8, i8, i8)> {
    rng(seed)
        .take(610)
        .map(price)
        .tuple_windows()
        .map(|(first, second)| (second - first))
        .tuple_windows()
}

fn rng(mut seed: u64) -> impl Iterator<Item = u64> {
    std::iter::from_fn(move || {
        let res = seed;
        seed = next_num(seed);
        Some(res)
    })
}

fn price(seed: u64) -> i8 {
    i8::try_from(seed % 10).expect("10 < i32::MAX")
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(1, 8_685_429)]
    fn test_nth(#[case] seed: u64, #[case] value: u64) {
        assert_eq!(rng(seed).nth(2000), Some(value));
    }

    #[rstest]
    #[case::example("example.txt", 23)]
    #[case::actual("part2.txt", 1501)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
