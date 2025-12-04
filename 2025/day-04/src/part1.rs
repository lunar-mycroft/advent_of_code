use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> usize {
    puzzle.reachable_stack().len()
}

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process_specialized(puzzle: Puzzle) -> usize {
    puzzle
        .grid
        .positions()
        .filter(|&center| match puzzle.grid.get(center).copied() {
            Some(b'@') => {
                crate::NEIGBORS
                    .iter()
                    .copied()
                    .map(|pos| pos + center)
                    .filter(|&pos| puzzle.grid.get(pos).is_some_and(|&b| b == b'@'))
                    .count()
                    < 4
            }
            Some(_) | None => false,
        })
        .count()
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;
    use tap::prelude::*;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 13)]
    #[case::puzzle("part1.txt", 1397)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: usize) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        assert_eq!(input.clone().pipe(process), expected);
        assert_eq!(process_specialized(input), expected);
        Ok(())
    }
}
