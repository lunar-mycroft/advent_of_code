use color_eyre::eyre::{bail, ensure};
use glam::IVec2;
use tap::prelude::*;

use common::grid::Grid;

pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
pub struct Puzzle {
    grid: Grid<u8>,
    start: i32,
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        ensure!(!s.contains("^^"), "Input contains adjacent splitters");
        let grid: Grid<u8> = s.parse()?;
        let width = grid.size().x;
        let start = width / 2;
        grid.positions()
            .map(|pos| (pos, grid[pos]))
            .try_fold((), |(), (pos, b)| match b {
                b'S' if pos == IVec2::new(start, 0) => Ok(()),
                b'S' => bail!("Start in wrong location: {pos}"),
                b'^' if pos.y % 2 == 1 => {
                    bail!("location {pos} not empty")
                }
                b'^' | b'.' => Ok(()),
                other => bail!("{other} is not a vaild byte"),
            })?;

        Self { grid, start }.pipe(Ok)
    }
}

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> (u64, u64) {
    #[inline]
    fn update_counts(
        splitters: &Grid<u8>,
        counts: &mut Grid<u64>,
        y: i32,
        mut op: impl FnMut(u64),
    ) -> u64 {
        debug_assert_eq!(splitters.size(), counts.size());
        let mut splits = 0;
        for x in 0..counts.size().x {
            let pos = IVec2::new(x, y);
            let paths = counts[pos];
            if paths == 0 {
                continue;
            }

            let new_pos = pos + IVec2::Y;
            match splitters[new_pos] {
                b'.' => {
                    counts[new_pos] += paths;
                    op(paths);
                }
                b'^' => {
                    counts[new_pos + IVec2::X] += paths;
                    counts[new_pos - IVec2::X] += paths;
                    op(paths * 2);
                    splits += 1;
                }
                _ => unreachable!(),
            }
        }
        splits
    }

    let mut counts: Grid<u64> = Grid::from_value(0, puzzle.grid.size());
    counts[IVec2::new(puzzle.start, 1)] = 1;

    let mut splits = 0;
    for y in 1..puzzle.grid.size().y - 2 {
        splits += update_counts(&puzzle.grid, &mut counts, y, drop);
    }
    let mut part_2 = 0;
    splits += update_counts(
        &puzzle.grid,
        &mut counts,
        puzzle.grid.size().y - 2,
        |amount| part_2 += amount,
    );
    (splits, part_2)
}

pub fn init_tracing() -> color_eyre::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

    let log_filter: EnvFilter = std::env::var(EnvFilter::DEFAULT_ENV)
        .map(String::leak)
        .map(|s| s as &str)
        .or_else(|err| match err {
            std::env::VarError::NotPresent => Ok("day_07=debug"),
            err @ std::env::VarError::NotUnicode(_) => Err(err),
        })?
        .parse()?;
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(log_filter)
        .with_line_number(true)
        .finish()
        .with(tracing_error::ErrorLayer::default());
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 21, 40)]
    #[case::part1("part1.txt", 1672, 231_229_866_702_355)]
    fn finds_solution(
        #[case] input_path: &str,
        #[case] part_1: u64,
        #[case] part_2: u64,
    ) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, (part_1, part_2));
        Ok(())
    }
}
