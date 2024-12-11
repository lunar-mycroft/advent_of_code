use itertools::Itertools;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(mut puzzle: Puzzle) -> usize {
    for _ in 0..25 {
        puzzle.stones = puzzle.stones.into_iter().flat_map(tick_stone).collect_vec();
    }
    puzzle.stones.len()
}

fn tick_stone(stone: u64) -> Vec<u64> {
    match stone {
        0 => vec![1],
        n if n.ilog(10) % 2 == 1 => {
            let s = n.to_string();
            let (a, b) = s.split_at(s.len() / 2);
            vec![
                a.parse().expect("will always be valid"),
                b.parse().expect("will always be valid"),
            ]
        }
        n => vec![n * 2024],
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use super::*;

    #[test]
    fn test_tick() {
        let initial = vec![0, 1, 10, 99, 999];
        let after = initial.into_iter().flat_map(tick_stone).collect_vec();
        assert_eq!(after, vec![1, 2024, 1, 0, 9, 9, 2_021_976]);
    }

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = "125 17".parse()?;
        let output = process(input);
        assert_eq!(output, 55312);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part1.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 218_079);
        Ok(())
    }
}
