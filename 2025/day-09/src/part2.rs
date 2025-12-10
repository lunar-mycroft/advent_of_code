use fxhash::FxHashMap;
use glam::IVec2;
use itertools::Itertools;
use tap::prelude::*;

use common::grid::Grid;

use crate::Puzzle;

// TODO: https://www.reddit.com/r/adventofcode/comments/1phywvn/2025_day_9_solutions/nt2nnxw/
#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(Puzzle { tiles }: Puzzle) -> Option<u64> {
    let shrunk = Shrunk::new(&tiles);
    let mut grid = Grid::from_value(
        (TileState::Unknown, 0u64),
        IVec2 {
            x: shrunk.xs.len().try_conv().expect("tiles.len() < i32::MAX"),
            y: shrunk.ys.len().try_conv().expect("tiles.len() < i32::MAX"),
        },
    );
    flood(&mut grid, &shrunk, &tiles);
    // compute summed area table
    for y in 1..grid.size().y {
        for x in 1..grid.size().y {
            let point = IVec2::new(x, y);
            let value = !matches!(grid[point].0, TileState::Outside);
            grid[point].1 = u64::from(value) + grid[point - IVec2::Y].1 + grid[point - IVec2::X].1
                - grid[point - IVec2::ONE].1;
        }
    }
    (0..tiles.len())
        .tuple_combinations()
        .filter_map(|(i, j)| -> Option<u64> {
            let (min, max) = {
                let (a, b) = (*shrunk.by_index.get(i)?, *shrunk.by_index.get(j)?);
                (a.min(b), a.max(b))
            };
            let expected = crate::area(min, max);
            let actual = grid[max].1 + grid.get(min - IVec2::ONE).map_or(0, |(_, v)| *v)
                - (grid
                    .get(IVec2::new(min.x - 1, max.y))
                    .map_or(0, |(_, v)| *v)
                    + grid
                        .get(IVec2::new(max.x, min.y - 1))
                        .map_or(0, |(_, v)| *v));
            if expected == actual {
                crate::area(tiles[i], tiles[j]).pipe(Some)
            } else {
                None
            }
        })
        .max()
}

pub fn flood(grid: &mut Grid<(TileState, u64)>, shrunk: &Shrunk, tiles: &[IVec2]) {
    for (i, j) in (0..tiles.len()).circular_tuple_windows() {
        let (start, end) = (shrunk.by_index[i], shrunk.by_index[j]);
        for y in start.y.min(end.y)..=start.y.max(end.y) {
            for x in start.x.min(end.x)..=start.x.max(end.x) {
                grid[IVec2::new(x, y)].0 = TileState::Inside;
            }
        }
    }
    let mut stack = (grid.size().x * grid.size().y)
        .try_conv()
        .map(Vec::with_capacity)
        .expect("area to be positive");
    stack.push(IVec2::ZERO);
    while let Some(point) = stack.pop() {
        for next in [IVec2::X, -IVec2::X, IVec2::Y, -IVec2::Y].map(|p| p + point) {
            match grid.get_mut(next) {
                Some((cell @ TileState::Unknown, _)) => {
                    *cell = TileState::Outside;
                    stack.push(next);
                }
                _ => (),
            }
        }
    }
}

#[cfg_attr(not(test), expect(unused))]
fn scan(grid: &mut Grid<(TileState, u64)>, shrunk: &Shrunk, tiles: &[IVec2]) {
    for (start, end) in (0..tiles.len())
        .circular_tuple_windows()
        .map(|(i, j)| (shrunk.by_index[i], shrunk.by_index[j]))
        .filter(|(start, end)| start.x == end.x)
    {
        if start.y < end.y {
            for y in start.y..end.y {
                grid[IVec2::new(end.x, y)].0 = TileState::Up;
            }
        } else {
            for y in end.y..start.y {
                grid[IVec2::new(end.x, y)].0 = TileState::Down;
            }
        }
    }
    for y in 0..grid.size().y {
        let mut direction = 0i8;
        for x in 0..grid.size().x {
            let point = IVec2::new(x, y);
            match grid[point].0 {
                TileState::Inside | TileState::Outside => (),
                TileState::Unknown if direction == 0 => grid[point].0 = TileState::Outside,
                TileState::Unknown => grid[point].0 = TileState::Inside,
                TileState::Up => direction = (direction + 1).clamp(-1, 1),
                TileState::Down => direction = (direction - 1).clamp(-1, 1),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TileState {
    Inside,
    Outside,
    Unknown,
    Up,
    Down,
}

pub struct Shrunk {
    xs: FxHashMap<i32, i32>,
    ys: FxHashMap<i32, i32>,
    by_index: Vec<IVec2>,
}

impl Shrunk {
    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    fn new(points: &[IVec2]) -> Self {
        let xs: FxHashMap<_, _> = {
            let mut xs = points.iter().map(|p| p.x).collect_vec();
            xs.extend([0, i32::MAX]);
            xs.sort_unstable();
            xs.dedup();
            xs.into_iter()
                .enumerate()
                .map(|(i, x)| (x, i as i32))
                .collect()
        };
        let ys: FxHashMap<_, _> = {
            let mut ys = points.iter().map(|p| p.y).collect_vec();
            ys.extend([0, i32::MAX]);
            ys.sort_unstable();
            ys.dedup();
            ys.into_iter()
                .enumerate()
                .map(|(i, y)| (y, i as i32))
                .collect()
        };
        Self {
            by_index: points
                .iter()
                .map(|p| IVec2::new(xs[&p.x], ys[&p.y]))
                .collect_vec(),
            xs,
            ys,
        }
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 24)]
    #[case::puzzle("input.txt", 1_476_550_548)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, Some(expected));
        Ok(())
    }

    #[rstest]
    #[ignore = "WIP"]
    #[case::example("example.txt")]
    #[ignore = "WIP"]
    #[case::puzzle("input.txt")]
    fn scan_correct(#[case] input_path: &str) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;

        let shrunk = Shrunk::new(&input.tiles);
        let mut flooded = Grid::from_value(
            (TileState::Unknown, 0u64),
            IVec2 {
                x: shrunk.xs.len().try_conv().expect("tiles.len() < i32::MAX"),
                y: shrunk.ys.len().try_conv().expect("tiles.len() < i32::MAX"),
            },
        );
        let mut scanned = Grid::from_value(
            (TileState::Unknown, 0u64),
            IVec2 {
                x: shrunk.xs.len().try_conv().expect("tiles.len() < i32::MAX"),
                y: shrunk.ys.len().try_conv().expect("tiles.len() < i32::MAX"),
            },
        );
        flood(&mut flooded, &shrunk, &input.tiles);
        scan(&mut scanned, &shrunk, &input.tiles);
        for pos in flooded.positions() {
            let (a, b) = (flooded[pos].0, scanned[pos].0);
            match (a, b) {
                (TileState::Inside, TileState::Inside | TileState::Up | TileState::Down)
                | (TileState::Outside, TileState::Outside)
                | (TileState::Unknown, TileState::Unknown | TileState::Inside) => (),
                (TileState::Inside, TileState::Outside | TileState::Unknown)
                | (
                    TileState::Outside,
                    TileState::Inside | TileState::Unknown | TileState::Up | TileState::Down,
                )
                | (TileState::Unknown, TileState::Outside | TileState::Up | TileState::Down) => {
                    panic!("{pos}: {a:?} != {b:?}")
                }
                (TileState::Up | TileState::Down, _) => unreachable!(),
            }
        }
        Ok(())
    }
}
