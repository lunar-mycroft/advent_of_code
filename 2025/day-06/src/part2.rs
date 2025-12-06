use color_eyre::eyre::{eyre, Result};
use common::grid::{self, Grid};
use glam::IVec2;
use itertools::Itertools;
use tap::prelude::*;

use crate::{Operator, Puzzle};

#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> Result<u64> {
    let size = IVec2 {
        x: puzzle
            .numbers
            .lines()
            .map(|s| s.trim_matches('\r'))
            .map(str::len)
            .unique()
            .at_most_one()
            .map_err(|_| grid::Error::InconsistentLines)?
            .ok_or(grid::Error::Empty)?
            .try_conv()?,
        y: puzzle.numbers.lines().count().try_conv()?,
    };
    let grid = Grid::from_row_major_ordered(
        puzzle
            .numbers
            .as_bytes()
            .iter()
            .copied()
            .filter(|&b| !matches!(b, b'\r' | b'\n')),
        size,
    );
    (0..grid.size().x)
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
        .chunk_by(|res| matches!(res, Ok(Some(_)) | Err(_)))
        .into_iter()
        .filter(|(key, _)| *key)
        .map(|(_, group)| group.filter_map_ok(std::convert::identity))
        .zip(puzzle.operations)
        .map(|(nums, op)| op.solve(nums))
        .pipe(|it| Operator::Add.solve(it))
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
        let input = common::read_input!(input_path).parse()?;
        let output = process(input)?;
        assert_eq!(output, expected);
        Ok(())
    }
}
