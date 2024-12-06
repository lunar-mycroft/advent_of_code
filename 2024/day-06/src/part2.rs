use std::collections::HashSet;

use color_eyre::eyre::{OptionExt as _, Result};
use glam::IVec2;
use itertools::Itertools;
use tap::prelude::*;

use crate::in_map;

use super::{blocks, guard, turn_right};

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
    let (mut guard_pos, mut guard_dir) = guard(input).ok_or_eyre("No player found")?;
    let mut visited = HashSet::new();
    // let obsticals = HashSet
    let mut res = 0;
    let mut n = 0;
    while in_map(guard_pos, size) {
        let old_dir = guard_dir;
        while blocks.contains(&(guard_pos + guard_dir)) {
            guard_dir = turn_right(guard_dir);
        }
        if blocks.contains(&(guard_pos + old_dir)) {
            let new_path = backtrack(guard_pos, guard_dir, &blocks)
                .take_while(|(pos, dir)| in_map(*pos, size) && !visited.contains(&(*pos, *dir)))
                .collect_vec();
            visited.extend(new_path);
        }
        if visited.contains(&(guard_pos, guard_dir)) {
            res += 1;
        }
        guard_pos += guard_dir;
    }
    dbg!(n);
    Ok(res)
}

fn backtrack<'a>(
    mut pos: IVec2,
    mut dir: IVec2,
    blocks: &HashSet<IVec2>,
) -> impl Iterator<Item = (IVec2, IVec2)> + '_ {
    std::iter::from_fn(move || {
        let old_dir = dir;
        while blocks.contains(&(pos - turn_right(dir))) {
            dir = -turn_right(dir);
        }
        let old_pos = pos;
        pos -= dir;
        Some((old_pos, old_dir))
    })
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
