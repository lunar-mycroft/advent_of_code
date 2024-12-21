use glam::IVec2;
use rayon::prelude::*;
use tap::prelude::*;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value, clippy::cast_sign_loss)]
pub fn process(puzzle: Puzzle) -> usize {
    let (costs, route) = puzzle.follow_route();
    route
        .into_par_iter()
        .flat_map(|pos| {
            [
                (pos, pos + IVec2::X),
                (pos, pos + IVec2::Y),
                (pos, pos - IVec2::X),
                (pos, pos - IVec2::Y),
                (pos, pos + (2 * IVec2::X)),
                (pos, pos + (2 * IVec2::Y)),
                (pos, pos - (2 * IVec2::X)),
                (pos, pos - (2 * IVec2::Y)),
                (pos, pos + IVec2::X + IVec2::Y),
                (pos, pos + IVec2::X - IVec2::Y),
                (pos, pos - IVec2::X + IVec2::Y),
                (pos, pos - IVec2::X - IVec2::Y),
            ]
        })
        .filter_map(|(b, e)| (costs.get(b)?, costs.get(e)?, (b - e).abs()).pipe(Some))
        .map(|(b, e, d)| (b, e, (d.x + d.y) as u32))
        .filter(|(b, e, _)| *b < *e)
        .filter(|(b, e, d)| *e - *b > *d)
        .map(|(b, e, d)| e - b - d)
        .filter(|saved| *saved >= 100)
        .count()
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 0);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part1.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 1367);
        Ok(())
    }
}
