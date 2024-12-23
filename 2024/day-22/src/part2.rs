use std::collections::{BTreeMap, HashMap, HashSet};

use itertools::Itertools;
use rayon::prelude::*;
use tap::prelude::*;

use crate::{Puzzle, Rng};

#[must_use]
pub fn initial(puzzle: &Puzzle) -> u64 {
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
            .take(610) // the sequences seem to repeat?
            .map(price)
            .tuple_windows()
            .map(|(first, second)| (second - first))
            .tuple_windows()
    }

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
pub fn one_pass(puzzle: &Puzzle) -> u64 {
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
pub fn u32_key(puzzle: &Puzzle) -> u64 {
    let mut cache: HashMap<u32, Entry> = HashMap::new();
    for (idx, seed) in puzzle.numbers.iter().copied().enumerate() {
        for (a, b, c, d, e) in Rng(seed).take(2001).map(price).tuple_windows() {
            cache
                .entry([b - a, c - b, c - d, e - d].pipe(u32_window))
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
pub fn fxhash_cache(puzzle: &Puzzle) -> u64 {
    use fxhash::FxHasher32;
    type Cache = HashMap<u32, Entry, std::hash::BuildHasherDefault<FxHasher32>>;
    let mut cache: Cache = Cache::default();
    for (idx, seed) in puzzle.numbers.iter().copied().enumerate() {
        for (a, b, c, d, e) in Rng(seed).take(2001).map(price).tuple_windows() {
            cache
                .entry([b - a, c - b, c - d, e - d].pipe(u32_window))
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
pub fn pre_alloc(puzzle: &Puzzle) -> u64 {
    // there are 40,951 total possible windows, so pre-allocate all of them
    let mut cache: HashMap<u32, Entry> = HashMap::with_capacity(40_951);
    for (idx, seed) in puzzle.numbers.iter().copied().enumerate() {
        for (a, b, c, d, e) in Rng(seed).take(2001).map(price).tuple_windows() {
            cache
                .entry([b - a, c - b, c - d, e - d].pipe(u32_window))
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
pub fn btree(puzzle: &Puzzle) -> u64 {
    // there are 40,951 total possible windows, so pre-allocate all of them
    let mut cache: BTreeMap<u32, Entry> = BTreeMap::new();
    for (idx, seed) in puzzle.numbers.iter().copied().enumerate() {
        for (a, b, c, d, e) in Rng(seed).take(2001).map(price).tuple_windows() {
            cache
                .entry([b - a, c - b, c - d, e - d].pipe(u32_window))
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
const fn pane(num: u32, prev: u32) -> u32 {
    let out = (num % 10 + 9) - (prev % 10);
    debug_assert!(out < 20);
    out
}

#[must_use]
pub fn continuous_windows(puzzle: &Puzzle) -> u64 {
    // there are 40,951 total possible windows, so pre-allocate all of them
    let mut cache: HashMap<u32, Entry> = HashMap::with_capacity(40_951);
    for (idx, seed) in puzzle.numbers.iter().copied().enumerate() {
        let mut rng = Rng(seed);
        let mut prev = rng.next().expect("rng not to end") % 10;
        let mut window = 0u32;
        for num in rng.by_ref().take(4) {
            window = (window << 5) | pane(num, prev);
            prev = num;
        }
        for num in rng.take(1997) {
            cache.entry(window).or_default().note_at(idx, price(prev));
            window = ((window << 5) | pane(num, prev)) & 0x000f_ffff;
            prev = num;
        }
    }

    cache
        .into_values()
        .map(|entry| entry.total_price)
        .max()
        .unwrap_or(0)
}

#[must_use]
pub fn vec_cache(puzzle: &Puzzle) -> u64 {
    #[inline]
    const fn pane(num: u32, prev: u32) -> u32 {
        (num % 10 + 9) - (prev % 10)
    }
    // there are 40,951 total possible windows, so pre-allocate all of them
    let mut cache = vec![None::<Entry>; 0xfffff];
    for (idx, seed) in puzzle.numbers.iter().copied().enumerate() {
        let mut rng = Rng(seed);
        let mut prev = rng.next().expect("rng not to end") % 10;
        let mut window = 0u32;
        for num in rng.by_ref().take(4) {
            window = (window << 5) | pane(num, prev);
            prev = num;
        }
        for num in rng.take(1997) {
            cache[window as usize]
                .get_or_insert_default()
                .note_at(idx, price(prev));
            window = ((window << 5) | pane(num, prev)) & 0x000f_ffff;
            prev = num;
        }
    }

    cache
        .into_iter()
        .flatten()
        .map(|entry| entry.total_price)
        .max()
        .unwrap_or(0)
}

#[must_use]
pub fn mul_windows(puzzle: &Puzzle) -> u64 {
    let mut cache = vec![None::<Entry>; 160_000];
    for (idx, seed) in puzzle.numbers.iter().copied().enumerate() {
        let mut rng = Rng(seed);
        let mut prev = rng.next().expect("rng not to end") % 10;
        let mut window = 0u32;
        for num in rng.by_ref().take(4) {
            window = (window * 20) + pane(num, prev);
            prev = num;
        }
        for num in rng.take(1997) {
            cache[window as usize]
                .get_or_insert_default()
                .note_at(idx, price(prev));
            window = ((window * 20) + pane(num, prev)) % 160_000;
            prev = num;
        }
    }

    cache
        .into_iter()
        .flatten()
        .map(|entry| entry.total_price)
        .max()
        .unwrap_or(0)
}

#[must_use]
pub fn small_cache(puzzle: &Puzzle) -> u64 {
    let mut cache = vec![None::<SmallEntry>; 160_000];
    for (idx, seed) in puzzle.numbers.iter().copied().enumerate() {
        let mut rng = Rng(seed);
        let mut prev = rng.next().expect("rng not to end") % 10;
        let mut window = 0u32;
        for num in rng.by_ref().take(4) {
            window = (window * 20) + pane(num, prev);
            prev = num;
        }
        for num in rng.take(1997) {
            cache[window as usize]
                .get_or_insert_default()
                .note_at(idx, price(prev));
            window = ((window * 20) + pane(num, prev)) % 160_000;
            prev = num;
        }
    }

    cache
        .into_iter()
        .flatten()
        .map(|entry| entry.total_price)
        .max()
        .unwrap_or(0)
        .into()
}

#[must_use]
#[allow(clippy::cast_sign_loss)]
pub fn small_cache_rayon(puzzle: &Puzzle) -> u64 {
    puzzle
        .numbers
        .par_iter()
        .copied()
        .map(|seed| {
            let mut seen = vec![false; 160_000];
            let mut cache = vec![0u16; 160_000];
            let mut rng = Rng(seed);
            let mut prev = rng.next().expect("rng not to end") % 10;
            let mut window = 0u32;
            for num in rng.by_ref().take(4) {
                window = (window * 20) + pane(num, prev);
                prev = num;
            }
            for num in rng.take(1997) {
                let idx = window as usize;
                if !seen[idx] {
                    cache[idx] += price(prev) as u16;
                    seen[idx] = true;
                }
                window = ((window * 20) + pane(num, prev)) % 160_000;
                prev = num;
            }
            cache
        })
        .reduce(
            || vec![0u16; 160_000],
            |mut acc, profits| {
                for idx in 0..160_000 {
                    acc[idx] += profits[idx];
                }
                acc
            },
        )
        .into_iter()
        .max()
        .unwrap_or(0)
        .into()
}

#[inline]
fn u32_window(deltas: [i8; 4]) -> u32 {
    deltas.map(|b| b.to_ne_bytes()[0]).pipe(u32::from_ne_bytes)
}

#[derive(Debug, Default, Clone, Copy)]
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

#[derive(Debug, Default, Clone, Copy)]
struct SmallEntry {
    total_price: u16,
    highest_seen: Option<u16>,
}

impl SmallEntry {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "known not to overflow"
    )]
    fn note_at(&mut self, idx: usize, price: i8) {
        debug_assert!(price >= 0);
        let idx = idx as u16;
        let price = price as u16;
        match self.highest_seen {
            None => {
                self.highest_seen = Some(idx);
                self.total_price += price;
            }
            Some(seen) if seen < idx => {
                self.highest_seen = Some(idx);
                self.total_price += price;
            }
            _ => (),
        }
    }
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
        assert_eq!(initial(&input), expected);
        assert_eq!(one_pass(&input), expected);
        assert_eq!(u32_key(&input), expected);
        assert_eq!(fxhash_cache(&input), expected);
        assert_eq!(pre_alloc(&input), expected);
        assert_eq!(btree(&input), expected);
        assert_eq!(continuous_windows(&input), expected);
        assert_eq!(vec_cache(&input), expected);
        assert_eq!(mul_windows(&input), expected);
        assert_eq!(small_cache(&input), expected);
        assert_eq!(small_cache_rayon(&input), expected);
        Ok(())
    }
}
