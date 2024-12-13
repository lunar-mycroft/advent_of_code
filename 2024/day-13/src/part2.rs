use crate::{Machine, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process_mat(puzzle: Puzzle) -> i64 {
    puzzle
        .machines
        .into_iter()
        .map(Machine::into_part_2)
        .filter_map(Machine::moves_to_win_mat)
        .map(|m| m.x * 3 + m.y)
        .sum()
}

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process_int(puzzle: Puzzle) -> i64 {
    puzzle
        .machines
        .into_iter()
        .map(Machine::into_part_2)
        .filter_map(Machine::moves_to_win_int)
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
        let output = process_mat(input);
        assert_eq!(output, 875_318_608_908);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part2.txt").parse()?;
        let output = process_mat(input);
        assert_eq!(output, 99_968_222_587_852);
        Ok(())
    }
}
