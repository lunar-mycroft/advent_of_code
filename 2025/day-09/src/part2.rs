use fxhash::FxHashMap;
use glam::IVec2;
use itertools::Itertools;
use tap::prelude::*;

use common::grid::Grid;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(Puzzle { tiles }: Puzzle) -> Option<u64> {
    let segments = tiles.iter().copied().circular_tuple_windows::<(_, _)>();

    let shrunk = Shrunk::new(&tiles);
    let mut grid = Grid::from_value(
        (None, 0u64),
        IVec2 {
            x: shrunk.xs.len().try_conv().expect("tiles.len() < i32::MAX"),
            y: shrunk.ys.len().try_conv().expect("tiles.len() < i32::MAX"),
        },
    );
    // fill with whether or not we're inside
    {
        for (start, end) in segments {
            let (start, end) = (
                shrunk.get(start).expect("point to be inside"),
                shrunk.get(end).expect("point to be inside"),
            );
            for y in start.y.min(end.y)..=start.y.max(end.y) {
                for x in start.x.min(end.x)..=start.x.max(end.x) {
                    grid[IVec2::new(x, y)].0 = Some(true);
                }
            }
        }
        let mut stack = (grid.size().x * grid.size().y)
            .try_conv()
            .map(Vec::with_capacity)
            .expect("area to be positive");
        stack.push(IVec2::ZERO);
        while let Some(point) = stack.pop() {
            for next in [
                point + IVec2::X,
                point - IVec2::X,
                point + IVec2::Y,
                point - IVec2::Y,
            ] {
                match grid.get_mut(next) {
                    Some((cell @ None, _)) => {
                        *cell = Some(false);
                        stack.push(next);
                    }
                    _ => (),
                }
            }
        }
    }
    // compute summed area table
    for y in 1..grid.size().y {
        for x in 1..grid.size().y {
            let point = IVec2::new(x, y);
            let value = grid[point].0 != Some(false);
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

    pub fn get(&self, point: IVec2) -> Option<IVec2> {
        IVec2 {
            x: *self.xs.get(&point.x)?,
            y: *self.ys.get(&point.y)?,
        }
        .pipe(Some)
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
}
