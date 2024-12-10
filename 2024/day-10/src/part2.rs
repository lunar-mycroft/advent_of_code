use glam::IVec2;

use crate::Puzzle;

#[allow(clippy::needless_pass_by_value)]
#[must_use]
pub fn process(puzzle: Puzzle) -> usize {
    puzzle
        .iter()
        .filter_map(|(pos, h)| (h == 0).then_some(pos))
        .map(|p| rank(p, &puzzle))
        .sum()
}

fn rank(trailhead: IVec2, map: &Puzzle) -> usize {
    match map.get(trailhead) {
        Some(9) => 1,
        Some(height) => [
            trailhead + IVec2::X,
            trailhead + IVec2::Y,
            trailhead - IVec2::X,
            trailhead - IVec2::Y,
        ]
        .into_iter()
        .filter(|p| map.get(*p) == Some(height + 1))
        .map(|p| rank(p, map))
        .sum(),
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = common::read_input!("example.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 81);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part2.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 1541);
        Ok(())
    }
}
