use std::ops::Deref;

use color_eyre::{eyre::OptionExt, Result};
use tap::prelude::*;

use crate::Puzzle;

pub fn process_big(puzzle: &Puzzle) -> u64 {
    let trie: TrieBig = puzzle.towels.iter().map(String::deref).collect();
    puzzle.goals.iter().map(|goal| trie.ways(goal)).sum()
}

pub fn process_small(puzzle: &Puzzle) -> u64 {
    let trie: TrieSmall = puzzle.towels.iter().map(String::deref).collect();
    puzzle.goals.iter().map(|goal| trie.ways(goal)).sum()
}

pub fn process_no_parse(s: &str) -> Result<u64> {
    let (towels, goals) = s
        .split_once("\n\n")
        .ok_or_eyre("Couldn't seperate blocks")?;
    let trie: TrieSmall = towels.split(", ").collect();
    goals
        .lines()
        .map(str::trim)
        .map(|goal| trie.ways(goal))
        .sum::<u64>()
        .pipe(Ok)
}

pub fn process_partial_inline(s: &str) -> Result<u64> {
    let (towels, goals) = s
        .split_once("\n\n")
        .ok_or_eyre("Couldn't seperate blocks")?;
    let trie: TrieSmall = towels.split(", ").collect();
    let mut sum = 0;
    for goal in goals.lines() {
        sum += trie.ways(goal.trim());
    }
    Ok(sum)
}

pub fn process_fully_inline(s: &str) -> Result<u64> {
    let (towels, goals) = s
        .split_once("\n\n")
        .ok_or_eyre("Couldn't seperate blocks")?;
    let trie: TrieSmall = towels.split(", ").collect();
    let mut sum = 0;
    for goal in goals.lines() {
        let this = &trie;
        let goal = goal.trim();
        let size = goal.len();
        let mut cache = [0; 80];
        cache[0] = 1;
        for start in 0..size {
            if cache[start] == 0 {
                continue;
            }
            let mut i = 0;

            for end in start..size {
                let hashed = goal.as_bytes()[end].pipe(TrieSmall::hash_fn);
                i = this.0[i].next[hashed];
                if i == 0 {
                    break;
                }

                cache[end + 1] += this.0[i].towels() * cache[start];
            }
        }
        sum += cache[size];
    }
    Ok(sum)
}

struct TrieBig(Vec<NodeBig>);

#[derive(Default, Debug)]
struct NodeBig {
    towel: bool,
    next: [usize; 23],
}

impl TrieBig {
    #[inline]
    fn hash_fn(c: u8) -> usize {
        usize::from(c - b'a')
    }

    #[inline]
    fn ways(&self, goal: &str) -> u64 {
        let size = goal.len();
        let mut cache = [0; 80];
        cache[0] = 1;
        for start in 0..size {
            if cache[start] == 0 {
                continue;
            }
            let mut i = 0;

            for end in start..size {
                let hashed = goal.as_bytes()[end].pipe(Self::hash_fn);
                i = self.0[i].next[hashed];
                if i == 0 {
                    break;
                }

                cache[end + 1] += if self.0[i].towel { cache[start] } else { 0 };
            }
        }
        cache[size]
    }
}

struct TrieSmall(Vec<NodeSmall>);

#[derive(Default, Debug)]
struct NodeSmall {
    next: [usize; 6],
}

impl TrieSmall {
    #[inline]
    fn hash_fn(c: u8) -> usize {
        usize::from((c ^ (c >> 4)) % 8)
    }

    #[inline]
    fn ways(&self, goal: &str) -> u64 {
        let size = goal.len();
        let mut cache = [0; 80];
        cache[0] = 1;
        for start in 0..size {
            if cache[start] == 0 {
                continue;
            }
            let mut i = 0;

            for end in start..size {
                let hashed = goal.as_bytes()[end].pipe(Self::hash_fn);
                i = self.0[i].next[hashed];

                if i == 0 {
                    break;
                }

                cache[end + 1] += self.0[i].towels() * cache[start];
            }
        }
        cache[size]
    }
}

impl NodeSmall {
    const fn towels(&self) -> u64 {
        self.next[3] as u64
    }

    fn set_towel(&mut self) {
        self.next[3] = 1;
    }
}

impl<'a> FromIterator<&'a str> for TrieBig {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut trie = Vec::with_capacity(1000);
        trie.push(NodeBig::default());
        for towel in iter {
            let mut i = 0;
            for c in towel.bytes().map(Self::hash_fn) {
                #[allow(clippy::map_entry)]
                if trie[i].next[c] != 0 {
                    i = trie[i].next[c];
                } else {
                    // new prefix;
                    let j = trie.len();
                    trie[i].next[c] = j;
                    i = j;
                    trie.push(NodeBig::default());
                }
            }
            trie[i].towel = true;
        }
        Self(trie)
    }
}

impl<'a> FromIterator<&'a str> for TrieSmall {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut trie = Vec::with_capacity(1000);
        trie.push(NodeSmall::default());
        for towel in iter {
            let mut i = 0;
            for c in towel.bytes().map(Self::hash_fn) {
                #[allow(clippy::map_entry)]
                if trie[i].next[c] != 0 {
                    i = trie[i].next[c];
                } else {
                    // new prefix;
                    let j = trie.len();
                    trie[i].next[c] = j;
                    i = j;
                    trie.push(NodeSmall::default());
                }
            }
            trie[i].set_towel();
        }
        Self(trie)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const TOWELS: [&str; 8] = ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];

    #[rstest]
    #[case("b", 1)]
    #[case("br", 2)]
    #[case("bbr", 2)]
    #[case("gbbr", 4)]
    #[case("rrbgbr", 6)]
    #[case("bwurrg", 1)]
    #[case("brgr", 2)]
    #[case("brwrr", 2)]
    fn test_ways(#[case] goal: &str, #[case] expected: u64) {
        let trie: TrieBig = TOWELS.iter().copied().collect();
        assert_eq!(trie.ways(goal), expected);
    }

    #[test]
    fn test_example() -> Result<()> {
        let input = common::read_input!("example.txt");
        let puzzle: Puzzle = common::read_input!("example.txt").parse()?;
        let big = process_big(&puzzle);
        let small = process_small(&puzzle);
        let no_parse = process_no_parse(&input)?;
        let partial_inline = process_partial_inline(&input)?;
        let fully_inline = process_fully_inline(&input)?;
        assert_eq!(big, 16);
        assert_eq!(small, 16);
        assert_eq!(no_parse, 16);
        assert_eq!(partial_inline, 16);
        assert_eq!(fully_inline, 16);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part2.txt");
        let puzzle: Puzzle = common::read_input!("part2.txt").parse()?;
        let big = process_big(&puzzle);
        let small = process_small(&puzzle);
        let no_parse = process_no_parse(&input)?;
        let partial_inline = process_partial_inline(&input)?;
        let fully_inline = process_fully_inline(&input)?;
        assert_eq!(big, 571_894_474_468_161);
        assert_eq!(small, 571_894_474_468_161);
        assert_eq!(no_parse, 571_894_474_468_161);
        assert_eq!(partial_inline, 571_894_474_468_161);
        assert_eq!(fully_inline, 571_894_474_468_161);
        Ok(())
    }
}
