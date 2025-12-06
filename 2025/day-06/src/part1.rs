use color_eyre::eyre::{ensure, eyre, Report, Result};
use itertools::Itertools;
use tap::prelude::*;

use crate::{Problem, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> u64 {
    puzzle
        .problems
        .iter()
        .map(|problem| match problem {
            Problem::Add(items) => items.iter().copied().sum::<u64>(),
            Problem::Mul(items) => items.iter().copied().product::<u64>(),
        })
        .sum()
}

impl Puzzle {
    pub fn parse_part_1(s: &str) -> Result<Self> {
        let mut rows = s
            .trim()
            .lines()
            .map(|line| line.split_whitespace().filter(|s| !s.is_empty()))
            .collect_vec();
        ensure!(rows.len() > 1, "Too few rows");

        Self {
            problems: std::iter::from_fn(|| {
                let mut strs = rows.iter_mut().rev().filter_map(Iterator::next);
                match strs.next()? {
                    "+" => strs
                        .map(str::parse::<u64>)
                        .try_collect()
                        .map(Problem::Add)
                        .map_err(Report::from)
                        .pipe(Some),
                    "*" => strs
                        .map(str::parse::<u64>)
                        .try_collect()
                        .map(Problem::Mul)
                        .map_err(Report::from)
                        .pipe(Some),
                    other => eyre!("{other:?} is not a valid operator")
                        .pipe(Err)
                        .pipe(Some),
                }
            })
            .try_collect()?,
        }
        .pipe(Ok)
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 4_277_556)]
    #[case::puzzle("part1.txt", 6_957_525_317_641)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input = common::read_input!(input_path).pipe_deref(Puzzle::parse_part_1)?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
