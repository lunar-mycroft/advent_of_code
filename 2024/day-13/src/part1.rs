use crate::{Machine, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> i64 {
    puzzle
        .machines
        .into_iter()
        .filter_map(Machine::moves_to_win)
        .filter(|m| m.max_element() <= 100)
        .map(|m| m.x * 3 + m.y)
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
        assert_eq!(output, 480);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part1.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 29_187);
        Ok(())
    }
}
