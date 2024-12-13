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
    use tap::prelude::*;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let output_mat = input.clone().pipe(process_mat);
        let output_int = process_int(input);
        assert_eq!(output_mat, 875_318_608_908);
        assert_eq!(output_int, 875_318_608_908);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part2.txt").parse()?;
        let output_mat = input.clone().pipe(process_mat);
        let output_int = process_int(input);
        assert_eq!(output_mat, 99_968_222_587_852);
        assert_eq!(output_int, 99_968_222_587_852);
        Ok(())
    }
}
