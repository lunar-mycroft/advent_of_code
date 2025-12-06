use color_eyre::eyre::{bail, eyre, OptionExt, Result};
use common::grid::{self, Grid};
use glam::IVec2;
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
        let (nums, ops) = s
            .trim()
            .rsplit_once('\n')
            .ok_or_eyre("Input only one row")?;
        let size = IVec2 {
            x: nums
                .lines()
                .map(|s| s.trim_matches('\r'))
                .map(str::len)
                .unique()
                .at_most_one()
                .map_err(|_| grid::Error::InconsistentLines)?
                .ok_or(grid::Error::Empty)?
                .try_conv()?,
            y: nums.lines().count().try_conv()?,
        };
        let grid = Grid::from_row_major_ordered(
            nums.as_bytes()
                .iter()
                .copied()
                .filter(|&b| !matches!(b, b'\r' | b'\n')),
            size,
        );
        Self {
            problems: (0..grid.size().x)
                .map(|x| {
                    (0..grid.size().y)
                        .map(|y| grid[IVec2::new(x, y)])
                        .filter_map(|d| match d {
                            d @ b'0'..=b'9' => Ok(d - b'0').pipe(Some),
                            b' ' => None,
                            other => Err(other).pipe(Some),
                        })
                        .map_ok(u64::from)
                        .try_fold(None, |opt, res| match (opt, res) {
                            (None, Ok(n)) => Some(n).pipe(Ok),
                            (_, Err(err)) => Err(err),
                            (Some(total), Ok(n)) => Some(total * 10 + n).pipe(Ok),
                        })
                        .map_err(|c| eyre!("{c} is not a valid digit"))
                })
                .chunk_by(|res| matches!(res, Ok(Some(_))))
                .into_iter()
                .filter(|(key, _)| *key)
                .map(|(_, group)| {
                    group
                        .filter_map_ok(std::convert::identity)
                        .try_collect::<_, Vec<_>, _>()
                })
                .zip(ops.split_whitespace())
                .map(|(res, op)| match op {
                    "+" => res.map(Problem::Add),
                    "*" => res.map(Problem::Mul),
                    other => bail!("{other:?} is not a valid operator"),
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
