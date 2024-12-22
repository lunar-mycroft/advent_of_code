use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use rayon::prelude::*;
use tap::prelude::*;

use crate::{Puzzle, Rng};

#[must_use]
pub fn initial(puzzle: &Puzzle) -> u64 {
    let all_sequences = puzzle
        .numbers
        .par_iter()
        .flat_map_iter(|n| sequences(*n))
        .collect::<HashSet<_>>();
    let all_buy_prices = puzzle.numbers.iter().copied().map(buy_prices).collect_vec();
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

#[must_use]
pub fn process_one_pass(puzzle: &Puzzle) -> u64 {
    let mut cache: HashMap<(i8, i8, i8, i8), Entry> = HashMap::new();
    for (idx, seed) in puzzle.numbers.iter().copied().enumerate() {
        for (a, b, c, d, e) in Rng(seed).take(2001).map(price).tuple_windows() {
            cache
                .entry((b - a, c - b, d - c, e - d))
                .or_default()
                .note_at(idx, e);
        }
    }

    cache
        .into_values()
        .map(|entry| entry.total_price)
        .max()
        .unwrap_or(0)
}

#[must_use]
pub fn process_int_key(puzzle: &Puzzle) -> u64 {
    let mut cache: HashMap<u32, Entry> = HashMap::new();
    for (idx, seed) in puzzle.numbers.iter().copied().enumerate() {
        for (a, b, c, d, e) in Rng(seed).take(2001).map(price).tuple_windows() {
            cache
                .entry([b - a, c - b, c - d, e - d].pipe(int_key))
                .or_default()
                .note_at(idx, e);
        }
    }

    cache
        .into_values()
        .map(|entry| entry.total_price)
        .max()
        .unwrap_or(0)
}

#[inline]
fn int_key(deltas: [i8; 4]) -> u32 {
    deltas.map(|b| b.to_ne_bytes()[0]).pipe(u32::from_ne_bytes)
}

#[derive(Debug, Default)]
struct Entry {
    total_price: u64,
    highest_seen: Option<usize>,
}

impl Entry {
    fn note_at(&mut self, idx: usize, price: i8) {
        match self.highest_seen {
            None => {
                self.highest_seen = Some(idx);
                self.total_price += u64::try_from(price).expect("prices to be positve");
            }
            Some(seen) if seen < idx => {
                self.highest_seen = Some(idx);
                self.total_price += u64::try_from(price).expect("prices to be positve");
            }
            _ => (),
        }
    }
}

fn buy_prices(seed: u32) -> HashMap<(i8, i8, i8, i8), i8> {
    Rng(seed).take(2001).map(price).tuple_windows().fold(
        HashMap::new(),
        |mut map, (a, b, c, d, e)| {
            let seq = (b - a, c - b, d - c, e - d);
            map.entry(seq).or_insert(e);
            map
        },
    )
}

fn sequences(seed: u32) -> impl Iterator<Item = (i8, i8, i8, i8)> {
    Rng(seed)
        .take(610)
        .map(price)
        .tuple_windows()
        .map(|(first, second)| (second - first))
        .tuple_windows()
}

fn price(seed: u32) -> i8 {
    i8::try_from(seed % 10).expect("10 < i32::MAX")
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 23)]
    #[case::actual("part2.txt", 1501)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        // assert_eq!(initial(&input), expected);
        assert_eq!(process_one_pass(&input), expected);
        assert_eq!(process_int_key(&input), expected);
        Ok(())
    }
}
