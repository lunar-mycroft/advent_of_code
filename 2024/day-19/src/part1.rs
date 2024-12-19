use std::collections::HashMap;

use crate::Puzzle;

/*
My idea, but needed spoilers to catch the bug.
Was only caching the outer call to is_possible, which of course did nothing
*/
#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> usize {
    let mut cache: HashMap<_, _> = HashMap::new();
    puzzle
        .goals
        .iter()
        .filter(|goal| is_possible(goal, &puzzle.towels, &mut cache))
        .count()
}

#[allow(clippy::option_if_let_else)]
fn is_possible<'s>(
    goal: &'s str,
    towels: &'s [String],
    cache: &mut HashMap<&'s str, bool>,
) -> bool {
    match cache.get(goal) {
        Some(b) => *b,
        None => {
            let res = towels.iter().any(|towel| match goal.strip_prefix(towel) {
                Some("") => true,
                Some(suffix) => is_possible(suffix, towels, cache),
                None => false,
            });
            cache.insert(goal, res);
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 6);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part1.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 287);
        Ok(())
    }
}
