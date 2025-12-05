use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> usize {
    use std::cmp::Ordering;
    puzzle
        .ids
        .iter()
        .copied()
        .filter(|id| {
            puzzle
                .ranges
                .binary_search_by(|range| match (range.start().cmp(id), range.end().cmp(id)) {
                    (Ordering::Less, Ordering::Less) => Ordering::Less,
                    (_, Ordering::Equal)
                    | (Ordering::Equal, _)
                    | (Ordering::Less, Ordering::Greater) => Ordering::Equal,
                    (Ordering::Greater, Ordering::Greater) => Ordering::Greater,
                    (Ordering::Greater, Ordering::Less) => unreachable!(),
                })
                .is_ok()
        })
        .count()
}

#[cfg(test)]
mod tests {

    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 3)]
    #[case::puzzle("part1.txt", 744)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: usize) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
