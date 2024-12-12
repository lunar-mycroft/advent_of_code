use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> usize {
    puzzle
        .regions()
        .map(|region| region.area * region.corners)
        .sum()
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 1_206);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let input: Puzzle = common::read_input!("example2.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 368);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part2.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 855_082);
        Ok(())
    }
}
