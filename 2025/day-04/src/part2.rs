use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(mut puzzle: Puzzle) -> usize {
    let (mut count, mut queue) = (0, puzzle.reachable_stack());
    while let Some(pos) = queue.pop() {
        if puzzle.grid[pos] != b'@' {
            continue;
        }
        count += 1;
        puzzle.grid[pos] = b'.';
        let exposed = crate::NEIGBORS
            .iter()
            .copied()
            .map(|offset| pos + offset)
            .filter(|&center| puzzle.reachable(center));
        queue.extend(exposed);
    }
    count
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 43)]
    #[case::puzzle("part2.txt", 8758)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: usize) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
