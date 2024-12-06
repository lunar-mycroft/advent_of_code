use std::collections::HashSet;

use color_eyre::eyre::{OptionExt as _, Result};
use glam::IVec2;
use itertools::Itertools;
use tap::prelude::*;

use crate::{in_map, patrol, turn_right};

use super::{blocks, guard};

#[allow(clippy::explicit_counter_loop)]
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
    let canidates = patrol(guard_pos, guard_dir, &blocks)
        .map(|(p, _)| p)
        .take_while(|p| in_map(*p, size))
        .unique();
    let mut res = 0usize;
    for canidate in canidates {
        let mut seen = HashSet::new();
        let mut new_blocks = blocks.clone();
        new_blocks.insert(canidate);
        let (mut pos, mut dir) = (guard_pos, guard_dir);
        while in_map(pos, size) && !seen.contains(&(pos, dir)) {
            seen.insert((pos, dir));
            if new_blocks.contains(&(pos + dir)) {
                dir = turn_right(dir);
            } else {
                pos += dir;
            }
        }
        if seen.contains(&(pos, dir)) {
            res += 1;
        }
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = common::read_input!("example.txt");
        let output = process(&input)?;
        assert_eq!(output, 6);
        Ok(())
    }

    // #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part2.txt");
        let output = process(&input)?;
        assert!(output > 526);
        assert_eq!(output, 0);
        Ok(())
    }
}
