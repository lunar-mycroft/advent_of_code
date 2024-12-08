use std::{collections::HashSet, iter::once};

use glam::IVec2;

use crate::Puzzle;

#[allow(clippy::needless_pass_by_value)]
#[must_use]
pub fn process(puzzle: Puzzle) -> usize {
    puzzle
        .iter()
        .filter_map(|(pos, h)| (h == 0).then_some(pos))
        .map(|p| score(p, &puzzle))
        .sum()
}

fn score(trailhead: IVec2, map: &Puzzle) -> usize {
    let (mut visited, mut fringe) = (
        once(trailhead).collect::<HashSet<_>>(),
        once(trailhead).collect::<HashSet<_>>(),
    );
    for height in 1..=9u8 {
        visited.extend(fringe.iter().copied());
        fringe = fringe
            .iter()
            .copied()
            .flat_map(|p| [p + IVec2::X, p + IVec2::Y, p - IVec2::X, p - IVec2::Y])
            .filter(|p| map.get(*p) == Some(height))
            .filter(|p| !visited.contains(p))
            .collect();
    }
    fringe.iter().copied().count()
}

#[cfg(test)]
mod tests {
    use color_eyre::Result;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = common::read_input!("example.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 36);
        Ok(())
    }

    #[test]
    fn test_example2() -> Result<()> {
        let input: Puzzle = common::read_input!("example2.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 2);
        Ok(())
    }

    #[test]
    fn test_example3() -> Result<()> {
        let input = common::read_input!("example3.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 4);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part1.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 746);
        Ok(())
    }
}
