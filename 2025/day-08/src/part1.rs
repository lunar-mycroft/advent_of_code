use tap::prelude::*;

use crate::{Dsu, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process((Puzzle { boxes }, by_distance): (Puzzle, Vec<(usize, usize)>)) -> usize {
    let mut dsu = boxes.len().pipe(Dsu::new);
    for &(u, v) in &by_distance {
        dsu.unite(u, v).expect("known valid indicies");
    }
    dsu.nodes.sort_unstable_by_key(|circuit| circuit.size);
    dsu.nodes
        .iter()
        .rev()
        .take(3)
        .map(|circuit| circuit.size)
        .product()
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 10, 40)]
    #[case::puzzle("part1.txt", 1000, 62_186)]
    fn finds_solution(
        #[case] input_path: &str,
        #[case] n: usize,
        #[case] expected: usize,
    ) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let by_distance = input.n_by_distance(n);
        let output = process((input, by_distance));
        assert_eq!(output, expected);
        Ok(())
    }
}
