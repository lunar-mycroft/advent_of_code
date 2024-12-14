use std::{cmp::Ordering, collections::HashMap};

use glam::IVec2;
use itertools::Itertools;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(mut puzzle: Puzzle, size: IVec2) -> usize {
    puzzle.robots.iter_mut().for_each(|robot| {
        for _ in 0..100 {
            robot.tick(size);
        }
    });
    puzzle
        .robots
        .into_iter()
        .map(|robot| robot.quadrent(size))
        .filter(|(h, v)| *h != Ordering::Equal && *v != Ordering::Equal)
        .fold(HashMap::new(), |mut map, k| {
            *map.entry(k).or_insert(0usize) += 1;
            map
        })
        .values()
        .product1()
        .expect("The iter to be non empty")
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("inputs")
                .join("example.txt"),
        )?
        .parse()?;
        let output = process(input, IVec2 { x: 11, y: 7 });
        assert_eq!(output, 12);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part1.txt").parse()?;
        let output = process(input, IVec2 { x: 101, y: 103 });
        assert_eq!(output, 230_461_440);
        Ok(())
    }
}
