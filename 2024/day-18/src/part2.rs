use glam::IVec2;

use crate::{astar, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process_astar(puzzle: Puzzle) -> IVec2 {
    let low = if puzzle.bytes.len() < 1024 { 12 } else { 1024 };
    let map = puzzle.map();

    for (idx, byte) in puzzle.bytes.iter().copied().enumerate().skip(low) {
        if astar(&map, idx + 1) == usize::MAX {
            return byte;
        }
    }
    panic!("No solution")
}

/*
A* terminates when it reaches it's goal || when there are no new nodes to explore.
Therefore, when not many positions are reachable from (0, 0), it terminates early.
This, combined with the fact that the last byte to fall is closer to the end of
the list, means that searching the list backwards is significantly faster than forwards
*/
#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process_astar_rev(puzzle: Puzzle) -> IVec2 {
    let map = puzzle.map();

    for (idx, byte) in puzzle.bytes.iter().copied().enumerate().rev() {
        if astar(&map, idx) != usize::MAX {
            return byte;
        }
    }
    panic!("No solution")
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use tap::prelude::*;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let astar = input.clone().pipe(process_astar);
        let rev = input.clone().pipe(process_astar_rev);
        assert_eq!(astar, IVec2::new(6, 1));
        assert_eq!(rev, IVec2::new(6, 1));
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part2.txt").parse()?;
        let astar = input.clone().pipe(process_astar);
        let rev = input.clone().pipe(process_astar_rev);
        assert_eq!(astar, IVec2::new(45, 16));
        assert_eq!(rev, IVec2::new(45, 16));
        Ok(())
    }
}
