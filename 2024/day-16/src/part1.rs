use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> u32 {
    let costs = puzzle.bfs();

    crate::DIRECTIONS
        .iter()
        .copied()
        .filter_map(|d| costs.get(puzzle.end, d))
        .min()
        .expect("a solution to be found")
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 7036);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part1.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 65436);
        Ok(())
    }
}
