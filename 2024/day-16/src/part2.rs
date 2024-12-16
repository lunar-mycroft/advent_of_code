use std::{
    collections::{hash_map, HashMap, HashSet},
    iter::once,
};

use color_eyre::eyre::Result;
use common::grid::Grid;
use glam::IVec2;
use itertools::Itertools;
use tap::prelude::*;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(mut puzzle: Puzzle) -> usize {
    let mut costs: HashMap<(IVec2, IVec2), usize> = HashMap::new();
    let mut parents: HashMap<(IVec2, IVec2), HashSet<(IVec2, IVec2)>> = HashMap::new();
    costs.insert((puzzle.start, D_0), 0);
    parents.insert((puzzle.start, D_0), HashSet::new());
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
                    parents.insert((new_p, new_d), [(pos, dir)].into());
                    if new_p == IVec2::new(13, 13) {}
                }
                hash_map::Entry::Occupied(mut entry) if *entry.get() == new_c => {
                    entry.insert(new_c);
                    stack.push((new_p, new_d, new_c));
                    parents
                        .entry((new_p, new_d))
                        .or_default()
                        .insert((pos, dir));
                    // if new_p == IVec2::new(13, 13) {
                    //     dbg!(pos);
                    // }
                }
                hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(new_c);
                    stack.push((new_p, new_d, new_c));
                    parents.insert((new_p, new_d), [(pos, dir)].into());
                    // if new_p == IVec2::new(13, 13) {
                    //     dbg!(pos);
                    // }
                }
                hash_map::Entry::Occupied(_) => (),
            }
        }
    }

    let mut i = 0;
    let mut visited = HashSet::new();
    let mut trace_stack = vec![(
        puzzle.end,
        DIRECTIONS
            .iter()
            .copied()
            .min_by_key(|d| costs.get(&(puzzle.end, *d)).copied().unwrap_or(usize::MAX))
            .expect("a solution"),
    )];
    while let Some(pos) = trace_stack.pop() {
        visited.insert(pos);
        // if i > 100 {
        //     break;
        // }
        i += 1;
        // if !parents.contains_key(&pos) {
        //     dbg!(pos);
        // }
        trace_stack.extend(
            parents
                .get(&pos)
                .into_iter()
                .flatten()
                .copied()
                .filter(|p| !visited.contains(p)),
        );
    }

    for (p, _) in &visited {
        puzzle.map[*p] = b'O';
    }
    for y in 0..puzzle.map.size().y {
        for x in 0..puzzle.map.size().x {
            print!(
                "{}",
                puzzle.map[IVec2::new(x, y)]
                    .conv::<u32>()
                    .pipe(char::from_u32)
                    .expect("")
            );
        }
        println!();
    }
    visited.into_iter().map(|(p, _)| p).unique().count()
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
