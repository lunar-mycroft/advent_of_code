use crate::Puzzle;

#[must_use]
pub fn process(mut puzzle: Puzzle) -> usize {
    let (mut left, mut right) = (0usize, puzzle.ids.len() - 1);
    while left < right {
        match (puzzle.ids[left], puzzle.ids[right]) {
            (_, None) => right -= 1,
            (None, Some(_)) => puzzle.ids.swap(left, right),
            (Some(_), Some(_)) => left += 1,
        }
    }
    puzzle.checksum()
}

#[cfg(test)]
mod tests {
    use color_eyre::Result;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = common::read_input!("example.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 1928);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part1.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 6_201_130_364_722);
        Ok(())
    }
}
