use crate::{Machine, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process_mat(puzzle: Puzzle) -> i64 {
    puzzle
        .machines
        .into_iter()
        .filter_map(Machine::moves_to_win_mat)
        .filter(|m| m.max_element() <= 100)
        .map(|m| m.x * 3 + m.y)
        .sum()
}

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process_int(puzzle: Puzzle) -> i64 {
    puzzle
        .machines
        .into_iter()
        .filter_map(Machine::moves_to_win_int)
        .filter(|m| m.max_element() <= 100)
        .map(|m| m.x * 3 + m.y)
        .sum()
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use tap::prelude::*;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let output_mat = input.clone().pipe(process_mat);
        let output_int = process_int(input);
        assert_eq!(output_mat, 480);
        assert_eq!(output_int, 480);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part1.txt").parse()?;
        let output_mat = input.clone().pipe(process_mat);
        let output_int = process_int(input);
        assert_eq!(output_mat, 29_187);
        assert_eq!(output_int, 29_187);
        Ok(())
    }
}
