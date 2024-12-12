use glam::IVec2;

use crate::{Puzzle, Region};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> usize {
    puzzle
        .regions()
        .map(|region| region.area() * region.perimiter())
        .sum()
}

impl Region {
    fn perimiter(&self) -> usize {
        self.0
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
                .filter(|v| !self.0.contains(v))
                .count()
            })
            .sum()
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
