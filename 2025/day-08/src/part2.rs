use glam::I64Vec3 as IVec3;
use itertools::Itertools as _;

use crate::{Dsu, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process((Puzzle { boxes }, by_distance): (Puzzle, Vec<(IVec3, IVec3, i64)>)) -> i64 {
    let mut dsu = Dsu::default();
    let mut n = 0;
    for (u, v, _) in by_distance {
        n += dsu.unite(u, v);
        if n == boxes.len() - 1 {
            return u.x * v.x;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 25_272)]
    #[case::puzzle("part2.txt", 8_420_405_530)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: i64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let by_distance = input.by_distance();
        let output = process((input, by_distance));
        assert_eq!(output, expected);
        Ok(())
    }
}
