use std::{collections::HashMap, ops::Deref};

use tap::prelude::*;

use crate::Puzzle;

struct Trie(Vec<Node>);

#[derive(Default, Debug)]
struct Node {
    towel: bool,
    next: HashMap<u8, usize>,
}

impl<'a> FromIterator<&'a str> for Trie {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut trie = Vec::with_capacity(1000);
        trie.push(Node::default());
        for towel in iter {
            let mut i = 0;
            for c in towel.bytes() {
                #[allow(clippy::map_entry)]
                if trie[i].next.contains_key(&c) {
                    i = trie[i].next[&c];
                } else {
                    // new prefix;
                    let j = trie.len();
                    trie[i].next.insert(c, j);
                    i = j;
                    trie.push(Node::default());
                }
            }
            trie[i].towel = true;
        }
        Self(trie)
    }
}

impl Trie {
    fn splits<'a>(&'a self, s: &'a str) -> impl Iterator<Item = (&'a str, &'a str)> + 'a {
        let mut i = Some(0);
        let mut n = 0;
        std::iter::from_fn(move || loop {
            let node = &self.0[i?];
            if n == s.len() && node.towel {
                i = None;
                break (s, "").pipe(Some);
            }
            i = node.next.get(s.as_bytes().get(n)?).copied();
            n += 1;
            if node.towel {
                break s.split_at(n - 1).pipe(Some);
            }
        })
    }
}

#[must_use]
#[allow(clippy::needless_pass_by_value, clippy::match_on_vec_items)]
pub fn process(puzzle: Puzzle) -> u64 {
    let trie: Trie = puzzle.towels.iter().map(String::deref).collect();

    puzzle
        .goals
        .iter()
        .map(|goal| {
            ways(goal, &trie, 0, &mut {
                let mut v = goal.len().pipe(Vec::with_capacity);
                v.resize(goal.len(), None);
                v
            })
        })
        .sum()
}

#[allow(clippy::option_if_let_else)]
fn ways(goal: &str, trie: &Trie, idx: usize, cache: &mut [Option<u64>]) -> u64 {
    let res = match (cache.get(idx).copied(), goal) {
        (_, "") => 1,
        (Some(Some(n)), _) => n,
        (Some(None), _) => {
            let res = trie
                .splits(goal)
                .map(|(towel, suffix)| {
                    // dbg!((towel, cache[idx + towel.len()]));
                    ways(suffix, trie, idx + towel.len(), cache)
                })
                .sum::<u64>();
            cache[idx] = Some(res);
            res
        }
        (None, _) => unreachable!(),
    };
    res
}

#[cfg(test)]
mod tests {
    use color_eyre::Result;
    use itertools::Itertools as _;
    use rstest::rstest;

    use super::*;

    const TOWELS: [&str; 8] = ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];

    #[rstest]
    #[case("gbbr", vec!["g", "gb"])]
    #[case("bbr", vec!["b"])]
    #[case("b", vec!["b"])]
    fn test_splits(#[case] s: &str, #[case] expected: Vec<&str>) {
        let trie: Trie = TOWELS.iter().copied().collect();
        let prefixes = trie.splits(s).map(|(s, _)| s).collect_vec();
        assert_eq!(prefixes, expected);
    }

    #[rstest]
    #[case("b", 1)]
    #[case("br", 2)]
    #[case("bbr", 2)]
    #[case("gbbr", 4)]
    #[case("rrbgbr", 6)]
    #[case("bwurrg", 1)]
    #[case("brgr", 2)]
    fn test_ways(#[case] goal: &str, #[case] expected: u64) {
        let trie: Trie = TOWELS.iter().copied().collect();
        assert_eq!(
            ways(goal, &trie, 0, &mut {
                let mut v = goal.len().pipe(Vec::with_capacity);
                v.resize(goal.len(), None);
                v
            }),
            expected
        );
    }

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 16);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part2.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 571_894_474_468_161);
        Ok(())
    }
}
