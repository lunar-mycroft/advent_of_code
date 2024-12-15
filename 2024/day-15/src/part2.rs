use tap::prelude::*;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> i32 {
    puzzle.wide().pipe(crate::part1::process)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use super::*;

    #[test]
    fn test_example2() -> Result<()> {
        let input: Puzzle = common::read_input!("example3.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 618);
        Ok(())
    }

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 9021);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part2.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 1_404_917);
        Ok(())
    }
}
