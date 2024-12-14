use common::grid::Grid;
use glam::IVec2;
use itertools::Itertools;
use tap::prelude::*;

use crate::Puzzle;

#[must_use]
pub fn process(mut puzzle: Puzzle, size: IVec2) -> usize {
    // horrible hack based on reddit comment,
    // wtf does "in the shape of a christmas tree **MEAN**"
    // if they'd _defined_ it I could have checked
    for i in 0usize.. {
        puzzle.robots.iter_mut().for_each(|r| r.tick(size));
        if all_unique(&puzzle) {
            return i + 1;
        }
    }

    unreachable!()
}

#[must_use]
pub fn process_grid_unique(mut puzzle: Puzzle, size: IVec2) -> usize {
    // horrible hack based on reddit comment,
    // wtf does "in the shape of a christmas tree **MEAN**"
    // if they'd _defined_ it I could have checked
    for i in 0usize.. {
        puzzle.robots.iter_mut().for_each(|r| r.tick(size));
        if grid_unique(&puzzle, size) {
            return i + 1;
        }
    }

    unreachable!()
}

#[must_use]
pub fn process_var(mut puzzle: Puzzle, size: IVec2) -> usize {
    #[allow(clippy::many_single_char_names)]
    fn egcd(a: i32, b: i32) -> (i32, i32, i32) {
        if a == 0 {
            (b, 0, 1)
        } else {
            let (g, x, y) = egcd(b % a, a);
            (g, y - (b / a) * x, x)
        }
    }

    fn mod_inverse(a: i32, m: i32) -> Option<i32> {
        let (g, x, _) = egcd(a, m);
        (g == 1).then_some((x % m + m) % m)
    }

    let min = {
        let mut min = IVec2::MAX;
        let mut index = IVec2::MAX;
        for i in 0..(size.max_element()) {
            let var = puzzle.variance();
            if var.x < min.x {
                min.x = var.x;
                index.x = i;
            }
            if var.y < min.y {
                min.y = var.y;
                index.y = i;
            }
            puzzle.robots.iter_mut().for_each(|r| r.tick(size));
        }
        index
    };
    let inv = mod_inverse(size.x, size.y).expect("103 and 101 to have a mod inverse");
    let m = (size.x * size.y)
        .try_conv::<usize>()
        .expect("size to be positive");
    (min.x + inv * (min.y - min.x) * size.x)
        .try_conv::<usize>()
        .expect("answer to be <= 103 * 101")
        % m
}

fn all_unique(puzzle: &Puzzle) -> bool {
    puzzle.robots.iter().map(|r| r.position).all_unique()
}

fn grid_unique(puzzle: &Puzzle, size: IVec2) -> bool {
    let mut grid = Grid::from_value(0u8, size);
    for pos in puzzle.robots.iter().map(|r| r.position) {
        if *grid.get(pos).expect("all robots to be in grid") != 0 {
            return false;
        }
        *grid.get_mut(pos).expect("all robots to be in grid") += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use super::*;

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part2.txt").parse()?;
        let size = IVec2 { x: 101, y: 103 };
        let out_hash = process(input.clone(), size);
        let out_grid = process_grid_unique(input.clone(), size);
        let out_var = process_var(input, size);
        assert_eq!(out_hash, 6668);
        assert_eq!(out_grid, 6668);
        assert_eq!(out_var, 6668);
        Ok(())
    }
}
