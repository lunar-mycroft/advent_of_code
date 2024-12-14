use common::grid::Grid;
use glam::IVec2;
use itertools::Itertools;

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
        let output = process(input, IVec2 { x: 101, y: 103 });
        assert_eq!(output, 6668);
        Ok(())
    }
}
