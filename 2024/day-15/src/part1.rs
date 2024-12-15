use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(mut puzzle: Puzzle) -> i32 {
    for dir in puzzle.moves.clone() {
        puzzle.step(puzzle.bot, dir);
    }
    puzzle.sum_coords()
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use super::*;

    #[test]
    fn test_sum() -> Result<()> {
        let input: Puzzle = r"
#######
#...O..
#.....@

^
"
        .trim()
        .parse()?;
        assert_eq!(input.sum_coords(), 104);
        Ok(())
    }

    #[test]
    fn test_example2() -> Result<()> {
        let input: Puzzle = common::read_input!("example2.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 2028);
        Ok(())
    }
    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 10_092);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part1.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 1_426_855);
        Ok(())
    }
}
