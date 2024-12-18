use glam::IVec2;

use crate::{astar, reachable, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process_astar(puzzle: Puzzle) -> IVec2 {
    let low = if puzzle.bytes.len() < 1024 { 12 } else { 1024 };
    let map = puzzle.map();

    for (idx, byte) in puzzle.bytes.iter().copied().enumerate().skip(low) {
        if !reachable(&map, idx + 1) {
            return byte;
        }
    }
    panic!("No solution")
}

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process_astar_binary(puzzle: Puzzle) -> IVec2 {
    let (mut lo, mut hi) = (
        if puzzle.bytes.len() < 1024 { 12 } else { 1024 },
        puzzle.bytes.len(),
    );
    let map = puzzle.map();

    let mut n = 0;
    while lo < hi - 1 {
        debug_assert!(n < 20);
        n += 1;
        let mid = (hi + lo) / 2;
        debug_assert_ne!(astar(&map, lo), usize::MAX, "lo unreachable");
        // debug_assert_eq!(astar(&map, hi), usize::MAX, "hi reachable");
        let reachable = reachable(&map, mid);
        if reachable {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    puzzle.bytes[if reachable(&map, hi) { hi } else { lo }]
}

// TODO: https://www.reddit.com/r/adventofcode/comments/1hguacy/2024_day_18_solutions/m2m7frf/

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

#[allow(clippy::redundant_clone)]
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
        let binary = input.clone().pipe(process_astar_binary);
        assert_eq!(astar, IVec2::new(6, 1));
        assert_eq!(rev, IVec2::new(6, 1));
        assert_eq!(binary, IVec2::new(6, 1));
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part2.txt").parse()?;
        let astar = input.clone().pipe(process_astar);
        let rev = input.clone().pipe(process_astar_rev);
        let binary = input.clone().pipe(process_astar_binary);
        assert_eq!(astar, IVec2::new(45, 16));
        assert_eq!(rev, IVec2::new(45, 16));
        assert_eq!(binary, IVec2::new(45, 16));
        Ok(())
    }
}
