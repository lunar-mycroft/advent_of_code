use glam::IVec2;

use crate::Puzzle;

#[allow(clippy::needless_pass_by_value)]
#[must_use]
pub fn process(puzzle: Puzzle) -> usize {
    puzzle
        .iter()
        .filter_map(|(pos, h)| (h == 0).then_some(pos))
        .map(|p| rank(p, &puzzle))
        .sum()
}

#[allow(clippy::needless_pass_by_value)]
#[must_use]
pub fn process_loop(puzzle: Puzzle) -> usize {
    puzzle
        .iter()
        .filter_map(|(pos, h)| (h == 0).then_some(pos))
        .map(|p| rank_loop(p, &puzzle))
        .sum()
}

fn rank(trailhead: IVec2, map: &Puzzle) -> usize {
    match map.get(trailhead) {
        Some(9) => 1,
        Some(height) => [
            trailhead + IVec2::X,
            trailhead + IVec2::Y,
            trailhead - IVec2::X,
            trailhead - IVec2::Y,
        ]
        .into_iter()
        .filter(|p| map.get(*p) == Some(height + 1))
        .map(|p| rank(p, map))
        .sum(),
        None => 0,
    }
}

fn rank_loop(trailhead: IVec2, map: &Puzzle) -> usize {
    let mut seen = 0;
    let mut stack = vec![trailhead];
    while let Some(current) = stack.pop() {
        match map.get(current) {
            Some(9) => {
                seen += 1;
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
    seen
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use tap::Pipe;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let recursive = input.clone().pipe(process);
        let looping = process_loop(input);
        assert_eq!(recursive, 81);
        assert_eq!(looping, 81);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part2.txt").parse()?;
        let recursive = input.clone().pipe(process);
        let looping = process_loop(input);
        assert_eq!(recursive, 1541);
        assert_eq!(looping, 1541);
        Ok(())
    }
}
