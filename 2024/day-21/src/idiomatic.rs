use std::collections::HashMap;

use common::grid::Grid;
use glam::IVec2;
use itertools::Itertools;
use tap::prelude::*;

use crate::Puzzle;

#[allow(dead_code)]
pub fn process(puzzle: &Puzzle, layers: u8) -> usize {
    let num_pad: (HashMap<_, _>, _) = {
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
        (
            g.positions()
                .zip(g.iter().copied())
                .map(|(v, c)| (c, v))
                .filter(|(c, _)| *c != '.')
                .collect(),
            g,
        )
    };
    let dir_pad: (HashMap<_, _>, _) = {
        let g: Grid<char> = r"
.^A
<v>
        "
        .trim()
        .parse()
        .expect("Known valid grid");
        (
            g.positions()
                .zip(g.iter().copied())
                .map(|(v, c)| (c, v))
                .filter(|(c, _)| *c != '.')
                .collect(),
            g,
        )
    };
    let num_routes = puzzle
        .codes
        .iter()
        .map(|(_, code)| routes(code, &num_pad).collect::<String>())
        .collect_vec();
    let robot_routes = std::iter::successors(
        num_routes
            .into_iter()
            .map(std::iter::once)
            .map(Counter::from_iter)
            .collect_vec()
            .pipe(Some),
        |robot_routes| {
            robot_routes
                .iter()
                .map(|route_counter| {
                    route_counter
                        .0
                        .iter()
                        .map(|(sub_route, n)| {
                            routes(sub_route, &dir_pad).collect::<Counter>() * (*n)
                        })
                        .fold(Counter::new(), |mut new_route, new_counts| {
                            new_route.update(new_counts);
                            new_route
                        })
                })
                .collect_vec()
                .pipe(Some)
        },
    )
    .nth(layers.into())
    .expect("Iterator to never end");
    debug_assert_eq!(robot_routes.len(), puzzle.codes.len());
    robot_routes
        .into_iter()
        .map(|route| route.0.into_iter().map(|(k, v)| k.len() * v).sum::<usize>())
        .zip(puzzle.codes.iter().map(|(n, _)| *n))
        .map(|(a, b)| a * b)
        .sum()
}

fn routes<'a>(
    path: &'a str,
    (pad, grid): &'a (HashMap<char, IVec2>, Grid<char>),
) -> impl Iterator<Item = String> + 'a {
    let mut start = 'A';
    path.chars().filter_map(move |end| {
        let old = start;
        start = end;
        step(old, end, pad, grid)
    })
}

#[derive(Debug, Clone, Default)]
struct Counter(HashMap<String, usize>);

impl Counter {
    fn new() -> Self {
        Self::default()
    }
    fn update(&mut self, other: Self) {
        for (key, value) in other.0 {
            *self.0.entry(key).or_insert(0) += value;
        }
    }
}

impl std::ops::MulAssign<usize> for Counter {
    fn mul_assign(&mut self, rhs: usize) {
        self.0.values_mut().for_each(|v| *v *= rhs);
    }
}

impl std::ops::Mul<usize> for Counter {
    type Output = Self;

    fn mul(mut self, rhs: usize) -> Self::Output {
        self *= rhs;
        self
    }
}

impl FromIterator<String> for Counter {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        iter.into_iter()
            .fold(HashMap::new(), |mut map, s| {
                *map.entry(s).or_insert(0) += 1;
                map
            })
            .pipe(Self)
    }
}

fn step(
    source: char,
    target: char,
    pad: &HashMap<char, IVec2>,
    grid: &Grid<char>,
) -> Option<String> {
    let (source, target) = (*pad.get(&source)?, *pad.get(&target)?);
    let delta = target - source;
    let vertical = match delta.y {
        ..0 => std::iter::repeat_n('^', usize::try_from(-delta.y).ok()?),
        0 => std::iter::repeat_n('!', 0),
        1.. => std::iter::repeat_n('v', usize::try_from(delta.y).ok()?),
    };
    let horizontal = match delta.x {
        ..0 => std::iter::repeat_n('<', usize::try_from(-delta.x).ok()?),
        0 => std::iter::repeat_n('!', 0),
        1.. => std::iter::repeat_n('>', usize::try_from(delta.x).ok()?),
    };
    match (
        delta.x,
        grid.get(IVec2 {
            x: source.x,
            y: target.y,
        })
        .is_some_and(|c| *c != '.'),
        grid.get(IVec2 {
            x: target.x,
            y: source.y,
        })
        .is_some_and(|c| *c != '.'),
    ) {
        (1.., true, _) => vertical.chain(horizontal),
        (_, _, true) => horizontal.chain(vertical),
        (_, true, _) => vertical.chain(horizontal),
        _ => unreachable!(),
    }
    .chain(std::iter::once('A'))
    .collect::<String>()
    .pipe(Some)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 2, 126_384)]
    #[case::example("example.txt", 25, 154_115_708_116_294)]
    #[case::actual("part1.txt", 2, 164_960)]
    #[case::actual("part1.txt", 25, 205_620_604_017_764)]
    fn finds_solution(
        #[case] input_path: &str,
        #[case] layers: u8,
        #[case] answer: usize,
    ) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        assert_eq!(process(&input, layers), answer);
        Ok(())
    }
}
