use std::collections::HashSet;

use glam::IVec2;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> usize {
    puzzle
        .regions()
        .map(|region| crate::area(&region) * perimiter(&region))
        .sum()
}

fn perimiter(region: &HashSet<IVec2>) -> usize {
    region
        .iter()
        .copied()
        .map(|pos| {
            [
                pos + IVec2::X,
                pos + IVec2::Y,
                pos - IVec2::X,
                pos - IVec2::Y,
            ]
            .into_iter()
            .filter(|v| !region.contains(v))
            .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 1930);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part1.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 1_433_460);
        Ok(())
    }
}
