use std::collections::{HashMap, HashSet};

use tap::prelude::*;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(Puzzle { connections }: Puzzle) -> u32 {
    let mut counts: HashMap<[u8; 3], u32> = connections.len().pipe(HashMap::with_capacity);
    let mut stack = vec![[b'y', b'o', b'u']];
    while let Some(machine) = stack.pop() {
        *counts.entry(machine).or_default() += 1;
        stack.extend(
            connections
                .get(&machine)
                .into_iter()
                .flat_map(HashSet::iter)
                .copied(),
        );
    }
    counts.get(b"out").copied().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 5)]
    #[case::puzzle("input.txt", 0)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u32) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
