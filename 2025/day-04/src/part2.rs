use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(mut puzzle: Puzzle) -> usize {
    let mut res = 0;
    loop {
        match puzzle.remove_reachable() {
            0 => break res,
            n => res += n,
        }
    }
}

impl Puzzle {
    fn remove_reachable(&mut self) -> usize {
        let mut res = 0;
        for center in self.grid.positions() {
            if self.reachable(center) {
                *self.grid.get_mut(center).expect("known to be inside") = b'.';
                res += 1;
            }
        }
        res
    }
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
