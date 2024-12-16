use std::collections::{hash_map, HashMap, HashSet};

use glam::IVec2;
use itertools::Itertools;
use tap::prelude::*;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(mut puzzle: Puzzle) -> usize {
    let mut costs: HashMap<(IVec2, IVec2), u32> = HashMap::new();
    costs.insert((puzzle.start, D_0), 0);
    let mut stack = vec![(puzzle.start, D_0, 0)];

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

    let end_dir = DIRECTIONS
        .iter()
        .copied()
        .min_by_key(|d| costs.get(&(puzzle.end, *d)).copied().unwrap_or(u32::MAX))
        .expect("a solution to be found");
    let mut stack = vec![(puzzle.end, end_dir)];
    let mut seen = HashSet::new();
    while let Some((pos, dir)) = stack.pop() {
        seen.insert(pos);
        let current_cost = costs[&(pos, dir)];
        if current_cost == 0 {
            continue;
        }
        let back = (pos - dir, dir);

        let t1 = (pos, left(dir));
        let t2 = (pos, -left(dir));
        match costs.get(&back).copied() {
            Some(cost) if cost == current_cost - 1 => {
                stack.push(back);
            }
            _ => (),
        }
        match costs.get(&t1).copied() {
            Some(cost) if cost == current_cost - 1000 => stack.push(t1),
            _ => (),
        }
        match costs.get(&t2).copied() {
            Some(cost) if cost == current_cost - 1000 => stack.push(t2),
            _ => (),
        }
    }
    seen.len()
}

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];
const D_0: IVec2 = IVec2::X;

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
    use color_eyre::eyre::Result;

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 45);
        Ok(())
    }

    // #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part2.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 1);
        Ok(())
    }
}
