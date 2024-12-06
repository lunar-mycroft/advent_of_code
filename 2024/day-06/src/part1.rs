use std::collections::HashSet;

use color_eyre::eyre::{OptionExt, Result};
use glam::IVec2;
use itertools::Itertools as _;
use tap::prelude::*;

use super::{blocks, guard, in_map, patrol};

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub fn process(input: &str) -> Result<usize> {
    let size = IVec2 {
        x: input.lines().count().try_conv::<i32>()?,
        y: input
            .lines()
            .next()
            .ok_or_eyre("empty input")?
            .chars()
            .count()
            .try_conv::<i32>()?,
    };
    let blocks = blocks(input).collect::<HashSet<_>>();
    let (guard_pos, guard_dir) = guard(input).ok_or_eyre("No player found")?;
    patrol(guard_pos, guard_dir, &blocks)
        .map(|(pos, _)| pos)
        .take_while(|pos| in_map(*pos, size))
        .unique()
        .count()
        .pipe(Ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = common::read_input!("example.txt");
        let output = process(&input)?;
        assert_eq!(output, 41);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part1.txt");
        let output = process(&input)?;
        assert_eq!(output, 4939);
        Ok(())
    }
}
