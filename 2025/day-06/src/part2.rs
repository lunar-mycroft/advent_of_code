use color_eyre::eyre::{bail, ensure, eyre, Report, Result};
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
    pub fn parse_part_2(s: &str) -> Result<Self> {
        #[derive(Debug, Clone, Copy)]
        enum Operator {
            Add,
            Mul,
        }
        let rows = s.trim().lines().collect_vec();
        ensure!(rows.len() > 1, "Input too short");
        ensure!(
            rows.iter().rev().skip(1).copied().map(str::len).all_equal(),
            "Non-rectangular input"
        );
        let last: Vec<_> = rows
            .last()
            .expect("Verified non-empty")
            .char_indices()
            .map(|(idx, c)| match c {
                ' ' => Ok(None),
                '+' => Some((idx, Operator::Add)).pipe(Ok),
                '*' => Some((idx, Operator::Mul)).pipe(Ok),
                c => bail!("{c:?} is not a valid character"),
            })
            .filter_map_ok(std::convert::identity)
            .try_collect()?;
        let digits = rows[..rows.len() - 1]
            .iter()
            .copied()
            .map(|s| {
                let mut v = last
                    .iter()
                    .tuple_windows::<(_, _)>()
                    .map(|(&(start, _), &(end, _))| &s[start..end - 1])
                    .collect_vec();
                let start = last.last().expect("Known non-empty").0;
                v.push(&s[start..]);
                v
            })
            .collect_vec();
        Self {
            problems: last
                .into_iter()
                .enumerate()
                .map(|(x, (_, op))| {
                    let digits = digits.iter().rev().map(|row| row[x]).collect_vec();
                    ensure!(digits.iter().copied().map(str::len).all_equal());
                    let width = digits.first().expect("Known non-empty").len();
                    let nums: Vec<_> = (0..width)
                        .map(|x| {
                            digits
                                .iter()
                                .rev()
                                .map(|b| b.as_bytes()[x])
                                .filter_map(|b| match b {
                                    digit @ b'0'..=b'9' => Ok(digit - b'0').pipe(Some),
                                    b' ' => None,
                                    other => {
                                        eyre!("{other} is not a valid digit").pipe(Err).pipe(Some)
                                    }
                                })
                                .map_ok(u64::from)
                                .try_fold(0, |acc, res| Ok::<_, Report>(acc * 10 + res?))
                        })
                        .try_collect()?;
                    match op {
                        Operator::Add => Problem::Add(nums),
                        Operator::Mul => Problem::Mul(nums),
                    }
                    .pipe(Ok)
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
    #[case::example("example.txt", 3_263_827)]
    #[case::puzzle("part2.txt", 13_215_665_360_076)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input = common::read_input!(input_path).pipe_deref(Puzzle::parse_part_2)?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
