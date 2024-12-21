use std::collections::HashMap;

use common::grid::Grid;
use glam::IVec2;
use itertools::Itertools;
use tap::prelude::*;

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
    let num_routes = puzzle
        .codes
        .iter()
        .map(|(_, code)| crate::part1::routes(code, &num_pad))
        .collect_vec();
    let mut robot_routes = num_routes
        .into_iter()
        .map(std::iter::once)
        .map(counter)
        .collect_vec();
    for _ in 0..25 {
        let mut new_routes = Vec::new();
        for route_counter in &robot_routes {
            let mut new_route = HashMap::new();
            for (sub_route, n) in route_counter {
                let mut new_counts = routes(sub_route, &dir_pad);
                let keys = new_counts.keys().cloned().collect_vec();
                for k in &keys {
                    *new_counts.get_mut(k).expect("key to be present") *= *n;
                }
                update_map(&mut new_route, new_counts);
            }
            new_routes.push(new_route);
        }
        robot_routes = new_routes;
    }
    debug_assert_eq!(robot_routes.len(), puzzle.codes.len());
    robot_routes
        .into_iter()
        .map(|route| route.into_iter().map(|(k, v)| k.len() * v).sum::<usize>())
        .zip(puzzle.codes.into_iter().map(|(n, _)| n))
        .map(|(a, b)| a * b)
        .sum()
}

fn update_map(lhs: &mut HashMap<String, usize>, rhs: HashMap<String, usize>) {
    for (key, value) in rhs {
        *lhs.entry(key).or_insert(0) += value;
    }
}

pub(crate) fn routes<'a>(path: &'a str, pad: &'a HashMap<char, IVec2>) -> HashMap<String, usize> {
    let mut start = 'A';
    path.chars()
        .filter_map(move |end| {
            let old = start;
            start = end;
            crate::step(old, end, pad)
        })
        .pipe(counter)
}

fn counter(it: impl IntoIterator<Item = String>) -> HashMap<String, usize> {
    it.into_iter().fold(HashMap::new(), |mut map, s| {
        *map.entry(s).or_insert(0) += 1;
        map
    })
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 154_115_708_116_294)]
    #[case::example("part2.txt", 205_620_604_017_764)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: usize) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
