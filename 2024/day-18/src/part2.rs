use common::grid::Grid;
use glam::IVec2;

use crate::{astar, is_passable_on, reachable, Puzzle};

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

// Binary search reduces the numbe of routes that need to be checked to ~11
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

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process_uf(mut puzzle: Puzzle) -> IVec2 {
    fn find(parents: &Grid<IVec2>, mut pos: IVec2) -> Option<IVec2> {
        loop {
            let parent = *parents.get(pos)?;
            if parent == pos {
                break Some(pos);
            }
            pos = parent;
        }
    }

    fn unite(parents: &mut Grid<IVec2>, a: IVec2, b: IVec2) {
        let a = find(parents, a).expect("didn't find a");
        let b = find(parents, b).expect("to find b");
        *parents.get_mut(a).expect("to find a in grid") = b;
    }

    fn unite_neighbors(
        parents: &mut Grid<IVec2>,
        pos: IVec2,
        map: &Grid<Option<usize>>,
        cutoff: usize,
    ) {
        for other in [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
            .into_iter()
            .map(|d| d + pos)
        {
            if map.get(other).is_none() {
                continue;
            }
            if is_passable_on(map, other, cutoff) && is_passable_on(map, pos, cutoff) {
                unite(parents, pos, other);
            }
        }
    }

    fn is_connected(parents: &Grid<IVec2>) -> bool {
        find(parents, IVec2::ZERO) == find(parents, parents.size() - IVec2::ONE)
    }

    let map = puzzle.map();
    let mut parents = Grid::from_positions(std::convert::identity, map.size());
    for pos in parents.positions() {
        unite_neighbors(&mut parents, pos, &map, puzzle.bytes.len());
    }
    debug_assert!(!is_connected(&parents));

    loop {
        let pos = puzzle
            .bytes
            .pop()
            .expect("The graph to become connected before the last byte is removed");

        unite_neighbors(&mut parents, pos, &map, puzzle.bytes.len());

        if is_connected(&parents) {
            break pos;
        }
    }
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
        let uf = input.clone().pipe(process_uf);
        assert_eq!(astar, IVec2::new(6, 1));
        assert_eq!(rev, IVec2::new(6, 1));
        assert_eq!(binary, IVec2::new(6, 1));
        assert_eq!(uf, IVec2::new(6, 1));
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
