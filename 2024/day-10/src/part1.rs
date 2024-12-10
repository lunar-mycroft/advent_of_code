use std::{collections::HashSet, iter::once};

use glam::IVec2;

use crate::Puzzle;

#[allow(clippy::needless_pass_by_value)]
#[must_use]
pub fn process(puzzle: Puzzle) -> usize {
    puzzle
        .iter()
        .filter_map(|(pos, h)| (h == 0).then_some(pos))
        .map(|p| score(p, &puzzle))
        .sum()
}

#[allow(clippy::needless_pass_by_value)]
#[must_use]
pub fn process_dfs(puzzle: Puzzle) -> usize {
    puzzle
        .iter()
        .filter_map(|(pos, h)| (h == 0).then_some(pos))
        .map(|p| score_dfs(p, &puzzle))
        .sum()
}

fn score(trailhead: IVec2, map: &Puzzle) -> usize {
    let (mut visited, mut fringe) = (
        once(trailhead).collect::<HashSet<_>>(),
        once(trailhead).collect::<HashSet<_>>(),
    );
    for height in 1..=9u8 {
        visited.extend(fringe.iter().copied());
        fringe = fringe
            .iter()
            .copied()
            .flat_map(|p| [p + IVec2::X, p + IVec2::Y, p - IVec2::X, p - IVec2::Y])
            .filter(|p| map.get(*p) == Some(height))
            .filter(|p| !visited.contains(p))
            .collect();
    }
    fringe.iter().copied().count()
}

fn score_dfs(trailhead: IVec2, map: &Puzzle) -> usize {
    let mut seen = HashSet::new();
    let mut stack = vec![trailhead];
    while let Some(current) = stack.pop() {
        match map.get(current) {
            Some(9) => {
                seen.insert(current);
            }
            Some(height) => {
                let successors = [
                    current + IVec2::X,
                    current + IVec2::Y,
                    current - IVec2::X,
                    current - IVec2::Y,
                ]
                .into_iter()
                .filter(|p| map.get(*p) == Some(height + 1));
                stack.extend(successors);
            }
            None => (),
        }
    }
    seen.len()
}

#[cfg(test)]
mod tests {
    use color_eyre::Result;
    use tap::prelude::*;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let bfs = input.clone().pipe(process);
        let dfs = process_dfs(input);
        assert_eq!(bfs, 36);
        assert_eq!(dfs, 36);
        Ok(())
    }

    #[test]
    fn test_example2() -> Result<()> {
        let input: Puzzle = common::read_input!("example2.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 2);
        Ok(())
    }

    #[test]
    fn test_example3() -> Result<()> {
        let input: Puzzle = common::read_input!("example3.txt").parse()?;
        let bfs = input.clone().pipe(process);
        let dfs = process_dfs(input);
        assert_eq!(bfs, 4);
        assert_eq!(dfs, 4);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part1.txt").parse()?;
        let bfs = input.clone().pipe(process);
        let dfs = process_dfs(input);
        assert_eq!(bfs, 746);
        assert_eq!(dfs, 746);
        Ok(())
    }
}
