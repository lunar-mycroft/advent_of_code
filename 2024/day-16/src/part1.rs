use std::collections::{hash_map, HashMap};

use color_eyre::eyre::Result;
use common::grid::Grid;
use glam::IVec2;
use tap::prelude::*;

use crate::Puzzle;

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> i64 {
    let mut costs: HashMap<(IVec2, IVec2), i64> = HashMap::new();
    costs.insert((puzzle.start, IVec2::NEG_Y), 0);
    let mut stack = vec![(puzzle.start, IVec2::X, 0)];

    while let Some((pos, dir, cost)) = stack.pop() {
        for (new_p, new_d, new_c) in [
            (pos + dir, dir, cost + 1),
            (pos, left(dir), cost + 1000),
            (pos, -left(dir), cost + 1000),
        ]
        .into_iter()
        .filter(|(p, _, _)| puzzle.map[*p] != b'#')
        {
            match costs.entry((new_p, new_d)) {
                hash_map::Entry::Occupied(mut entry) if *entry.get() > new_c => {
                    entry.insert(new_c);
                    stack.push((new_p, new_d, new_c));
                }
                hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(new_c);
                    stack.push((new_p, new_d, new_c));
                }
                hash_map::Entry::Occupied(_) => (),
            }
        }
    }

    DIRECTIONS
        .iter()
        .copied()
        .filter_map(|d| costs.get(&(puzzle.end, d)))
        .copied()
        .min()
        .expect("a solution to be found")
}

fn left(dir: IVec2) -> IVec2 {
    match dir {
        IVec2 { x: 0, y: -1 } => -IVec2::X,
        IVec2 { x: 0, y: 1 } => IVec2::X,
        IVec2 { x: 1, y: 0 } => -IVec2::Y,
        IVec2 { x: -1, y: 0 } => IVec2::Y,
        _ => panic!("Called turn on invalid direction"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 7036);
        Ok(())
    }

    // #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part1.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 0);
        Ok(())
    }
}
