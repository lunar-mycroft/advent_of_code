use glam::IVec2;
use itertools::Itertools;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(mut puzzle: Puzzle, size: IVec2) -> usize {
    // horrible hack based on reddit comment,
    // wtf does "in the shape of a christmas tree **MEAN**"
    for i in 0usize.. {
        puzzle.robots.iter_mut().for_each(|r| r.tick(size));
        if puzzle.robots.iter().map(|r| r.position).all_unique() {
            return i + 1;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use super::*;

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part2.txt").parse()?;
        let output = process(input, IVec2 { x: 101, y: 103 });
        assert_eq!(output, 6668);
        Ok(())
    }
}
