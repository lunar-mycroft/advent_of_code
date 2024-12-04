use std::collections::HashSet;

use color_eyre::eyre::{ensure, Result};
use itertools::Itertools;
use tap::prelude::*;

pub fn process(input: &str) -> Result<usize> {
    let lines = input.lines().collect_vec();
    ensure!(
        lines[0].len() == lines.len(),
        "Non square inputs not allowed"
    );
    let width = lines[0].len();
    let down_right = (0..width)
        .flat_map(|x| {
            lines
                .iter()
                .enumerate()
                .take(width - x)
                .map(|(y, line)| &line[(y + x)..=(y + x)])
                .collect::<String>()
                .pipe_deref(mas_indexes)
                .map(|y| (x + 1, y))
                .collect_vec()
        })
        .chain((1..width).flat_map(|y| {
            (0..(width - y))
                .map(|x| (x, x + y))
                .map(|(x, y)| &lines[y][x..=x])
                .collect::<String>()
                .pipe_deref(mas_indexes)
                .map(|idx| (idx, y + idx))
                .collect_vec()
        }))
        .collect::<HashSet<_>>();
    let down_left = (0..width)
        .flat_map(|x| {
            lines
                .iter()
                .enumerate()
                .take(width - x)
                .map(|(y, line)| {
                    let idx = width - y - x - 1;
                    &line[idx..=idx]
                })
                .collect::<String>()
                // .pipe(|s| dbg!(s))
                .pipe_deref(mas_indexes)
                .map(|idx| (width - x - 1 - idx, idx))
                .collect_vec()
        })
        .chain((1..width).flat_map(|y| {
            (0..(width - y))
                .map(|x| (width - x - 1, x + y))
                .map(|(x, y)| &lines[y][x..=x])
                .collect::<String>()
                .pipe_deref(mas_indexes)
                .map(|idx| (width - 1 - idx, y + idx))
                .collect_vec()
        }))
        .collect::<HashSet<_>>();
    let intersect = down_left.intersection(&down_right).sorted().collect_vec();
    dbg!(intersect);
    todo!()
}

fn mas_indexes(s: &str) -> impl Iterator<Item = usize> + '_ {
    s.match_indices("MAS")
        .map(|(idx, _)| idx + 1)
        .chain(s.match_indices("SAM").map(|(idx, _)| idx + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn test_example() -> Result<()> {
        let input = common::read_input!("example.txt");
        let output = process(&input)?;
        assert_eq!(output, 9);
        Ok(())
    }

    // #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part2.txt");
        let output = process(&input)?;
        assert_eq!(output, 0);
        Ok(())
    }
}
