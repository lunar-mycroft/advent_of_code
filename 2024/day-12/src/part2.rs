use std::collections::{HashMap, HashSet};

use glam::IVec2;
use itertools::Itertools;

use crate::{Puzzle, Region};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> usize {
    puzzle
        .regions()
        .map(|region| {
            region.area()
                * region
                    .0
                    .iter()
                    .copied()
                    .map(|pos| region.corners(pos))
                    .sum::<usize>()
        })
        .sum()
}

impl Region {
    #[expect(unused)]
    fn sides(&self) -> usize {
        let segs: HashMap<(IVec2, i32), HashSet<i32>> = self
            .0
            .iter()
            .copied()
            .flat_map(|pos| {
                [
                    (pos, IVec2::X),
                    (pos, IVec2::Y),
                    (pos, IVec2::NEG_X),
                    (pos, IVec2::NEG_Y),
                ]
                .into_iter()
                .filter(|(p, d)| !self.0.contains(&(*p + *d)))
            })
            .fold(HashMap::new(), |mut map, (pos, dir)| {
                let (key, val) = match dir {
                    IVec2 { x: 1 | -1, y: 0 } => ((dir, pos.x), pos.y),
                    IVec2 { x: 0, y: 1 | -1 } => ((dir, pos.y), pos.x),
                    _ => unreachable!(),
                };
                map.entry(key).or_default().insert(val);
                map
            });
        segs.into_values()
            .map(|set| set.into_iter().sorted_unstable().collect_vec())
            .map(|v| {
                v.into_iter()
                    .tuple_windows()
                    .filter(|(a, b)| *b - *a > 1)
                    .count()
                    + 1
            })
            .sum()
    }

    fn corners(&self, pos: IVec2) -> usize {
        [IVec2::NEG_Y, IVec2::X, IVec2::Y, IVec2::NEG_X]
            .into_iter()
            .cycle()
            .tuple_windows()
            .take(4)
            .map(|(a, b)| (pos + a, pos + b, pos + a + b))
            .map(|(left, right, mid)| {
                (
                    self.0.contains(&left),
                    self.0.contains(&right),
                    self.0.contains(&mid),
                )
            })
            .filter(|(l_in, r_in, m_in)| (!l_in && !r_in) || (*l_in && *r_in && !m_in))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 1_206);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let input: Puzzle = common::read_input!("example2.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 368);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part2.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 855_082);
        Ok(())
    }
}
