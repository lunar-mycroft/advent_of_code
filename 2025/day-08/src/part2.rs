use tap::prelude::*;

use crate::{Dsu, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process((Puzzle { boxes }, by_distance): (Puzzle, Vec<(usize, usize)>)) -> i64 {
    let mut dsu = boxes.len().pipe(Dsu::new);
    for (u, v) in by_distance {
        if dsu.unite(u, v).expect("known valid indicies") == boxes.len() {
            return boxes[u].x * boxes[v].x;
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
