use std::collections::HashMap;

use common::grid::Grid;
use glam::IVec2;
use itertools::Itertools as _;
use tap::prelude::*;

use crate::Puzzle;

#[allow(dead_code)]
pub fn process(puzzle: &Puzzle, layers: u8) -> usize {
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
        .map(|(_, code)| routes(code, &num_pad).collect::<String>())
        .collect_vec();
    let mut robot_routes = num_routes
        .into_iter()
        .map(std::iter::once)
        .map(counter)
        .collect_vec();
    for _ in 0..layers {
        let mut new_routes = Vec::new();
        for route_counter in &robot_routes {
            let mut new_route = HashMap::new();
            for (sub_route, n) in route_counter {
                let mut new_counts = routes(sub_route, &dir_pad).pipe(counter);
                new_counts.values_mut().for_each(|v| *v *= n);
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
        .zip(puzzle.codes.iter().map(|(n, _)| *n))
        .map(|(a, b)| a * b)
        .sum()
}

fn update_map(lhs: &mut HashMap<String, usize>, rhs: HashMap<String, usize>) {
    for (key, value) in rhs {
        *lhs.entry(key).or_insert(0) += value;
    }
}

fn routes<'a>(path: &'a str, pad: &'a HashMap<char, IVec2>) -> impl Iterator<Item = String> + 'a {
    let mut start = 'A';
    path.chars().filter_map(move |end| {
        let old = start;
        start = end;
        step(old, end, pad)
    })
}

fn counter(it: impl IntoIterator<Item = String>) -> HashMap<String, usize> {
    it.into_iter().fold(HashMap::new(), |mut map, s| {
        *map.entry(s).or_insert(0) += 1;
        map
    })
}

fn step(source: char, target: char, pad: &HashMap<char, IVec2>) -> Option<String> {
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
    if delta.x > 0
        && pad.values().contains(&IVec2 {
            x: source.x,
            y: target.y,
        })
    {
        let s = vertical
            .chain(horizontal)
            .chain(std::iter::once('A'))
            .collect();
        return Some(s);
    }
    if pad.values().contains(&IVec2 {
        x: target.x,
        y: source.y,
    }) {
        let s = horizontal
            .chain(vertical)
            .chain(std::iter::once('A'))
            .collect();
        return Some(s);
    }
    if pad.values().contains(&IVec2 {
        x: source.x,
        y: target.y,
    }) {
        return Some(
            vertical
                .chain(horizontal)
                .chain(std::iter::once('A'))
                .collect(),
        );
    }
    unreachable!()
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
