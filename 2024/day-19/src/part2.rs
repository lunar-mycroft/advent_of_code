use std::collections::HashMap;

use tap::prelude::*;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> u64 {
    let mut cache: HashMap<_, _> = HashMap::new();
    puzzle
        .goals
        .iter()
        .map(|goal| crate::ways(goal, &puzzle.towels, &mut cache))
        .sum()
}

/*
improvements inspired by
https://www.reddit.com/r/adventofcode/comments/1hhlb8g/2024_day_19_solutions/m2s6aev/
*/

#[must_use]
#[allow(
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::match_on_vec_items
)]
pub fn process_vec_cache(puzzle: Puzzle) -> u64 {
    fn ways(goal: &str, towels: &[String], idx: usize, cache: &mut Vec<Option<u64>>) -> u64 {
        match cache[idx] {
            Some(n) => n,
            None => {
                // let res = towels
                //     .iter()
                //     .map(|towel| match goal.strip_prefix(towel) {
                //         Some("") => 1,
                //         Some(suffix) => ways(suffix, towels, idx + towel.len(), cache),
                //         None => 0,
                //     })
                //     .sum();
                let mut res = 0;
                for towel in towels {
                    match goal.strip_prefix(towel) {
                        Some("") => res += 1,
                        Some(suffix) => res += ways(suffix, towels, idx + towel.len(), cache),
                        None => (),
                    }
                }
                cache[idx] = Some(res);
                res
            }
        }
    }

    puzzle
        .goals
        .iter()
        .map(|goal| {
            ways(goal, &puzzle.towels, 0, &mut {
                let mut v = goal.len().pipe(Vec::with_capacity);
                v.resize(goal.len(), None);
                v
            })
        })
        .sum()
}

#[allow(clippy::redundant_clone)]
#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use super::*;

    #[test]
    #[ignore]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let naive = input.clone().pipe(process);
        let vec_cache = input.clone().pipe(process_vec_cache);
        assert_eq!(naive, 16);
        assert_eq!(vec_cache, 16);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part2.txt").parse()?;
        let naive = input.clone().pipe(process);
        let vec_cache = input.clone().pipe(process_vec_cache);
        assert_eq!(naive, 571_894_474_468_161);
        assert_eq!(vec_cache, 571_894_474_468_161);
        Ok(())
    }
}
