use common::grid::Grid;
use glam::IVec2;

use crate::{astar, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> usize {
    let size = IVec2::ONE * if puzzle.bytes.len() < 1_024 { 7 } else { 71 };
    let mut grid = Grid::from_value(false, size);
    for byte in puzzle
        .bytes
        .iter()
        .copied()
        .take(if puzzle.bytes.len() < 1024 { 12 } else { 1024 })
    {
        grid[byte] = true;
    }

    astar(&grid, size)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 22);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part1.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 408);
        Ok(())
    }
}
