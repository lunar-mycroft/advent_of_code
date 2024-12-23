use glam::IVec2;
use itertools::Itertools;
use tap::prelude::*;

use crate::Puzzle;

#[must_use]
#[allow(
    clippy::needless_pass_by_value,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]
pub fn process(puzzle: Puzzle) -> usize {
    let offsets = (-20i32..=20)
        .cartesian_product(-20i32..=20)
        .filter_map(|(x, y)| {
            if (0..=20).contains(&(x.abs() + y.abs())) {
                IVec2::new(x, y).pipe(Some)
            } else {
                None
            }
        })
        .collect_vec();
    let (costs, route) = puzzle.follow_route();
    debug_assert_eq!(costs.get(puzzle.end), Some((route.len() - 1) as u32));
    route
        .into_iter()
        .flat_map(|pos| offsets.iter().copied().map(move |o| (pos, pos + o)))
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
    #[ignore]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 0);
        Ok(())
    }

    #[test]
    // #[ignore]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part2.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 1_006_850);
        Ok(())
    }
}
