use itertools::Itertools as _;

use crate::{Dsu, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(Puzzle { boxes }: Puzzle) -> i64 {
    let by_distance = {
        let mut res = boxes
            .iter()
            .tuple_combinations::<(_, _)>()
            .map(|(a, b)| (*a, *b, a.distance_squared(*b)))
            .collect_vec();
        res.sort_unstable_by_key(|(_, _, r)| *r);
        res
    };
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
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
