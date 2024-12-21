use std::collections::HashMap;

use common::grid::Grid;
use glam::IVec2;
use itertools::Itertools;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> usize {
    let num_pad: HashMap<_, _> = {
        let g: Grid<char> = r"
789
456
123
.0A
        "
        .trim()
        .parse()
        .expect("Known valid grid");
        debug_assert_eq!(g.size(), IVec2::new(3, 4));
        g.positions()
            .zip(g)
            .map(|(v, c)| (c, v))
            .filter(|(c, _)| *c != '.')
            .collect()
    };
    debug_assert!(!num_pad.values().contains(&IVec2::new(0, 3)));
    let dir_pad: HashMap<_, _> = {
        let g: Grid<char> = r"
.^A
<v>
        "
        .trim()
        .parse()
        .expect("Known valid grid");
        debug_assert_eq!(g.size(), IVec2::new(3, 2));
        g.positions()
            .zip(g)
            .map(|(v, c)| (c, v))
            .filter(|(c, _)| *c != '.')
            .collect()
    };
    let cold_routes = puzzle
        .codes
        .iter()
        .map(|(_, code)| routes(code, &num_pad))
        .map(|route| routes(&route, &dir_pad))
        .map(|route| routes(&route, &dir_pad))
        .collect_vec();
    puzzle
        .codes
        .iter()
        .map(|(n, _)| *n)
        .zip(cold_routes)
        .map(|(n, route)| route.len() * n)
        .sum()
}

pub(crate) fn routes<'a>(path: &'a str, pad: &'a HashMap<char, IVec2>) -> String {
    let mut start = 'A';
    path.chars()
        .filter_map(move |end| {
            let old = start;
            start = end;
            crate::step(old, end, pad)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 126_384)]
    #[case::actual("part1.txt", 164_960)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: usize) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
