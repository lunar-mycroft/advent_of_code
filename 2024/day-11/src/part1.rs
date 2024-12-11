use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> usize {
    puzzle.simulate(25)
}

#[must_use]
#[allow(clippy::needless_pass_by_value)]
#[inline]
pub fn process_no_alloc(puzzle: Puzzle) -> usize {
    puzzle.simulate_no_alloc(25)
}

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process_sum(puzzle: Puzzle) -> usize {
    puzzle.depth_first(25, stones_after)
}

fn stones_after(stone: u64, steps: u8) -> usize {
    match stone {
        _ if steps == 0 => 1,
        0 => stones_after(1, steps - 1),
        n if n.ilog(10) % 2 == 1 => {
            let s = n.to_string();
            let (a, b) = s.split_at(s.len() / 2);

            stones_after(a.parse().expect("will always be valid"), steps - 1)
                + stones_after(b.parse().expect("will always be valid"), steps - 1)
        }
        n => stones_after(n * 2024, steps - 1),
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use itertools::Itertools;
    use tap::prelude::*;

    use super::*;

    #[test]
    fn test_tick() {
        let initial = vec![0, 1, 10, 99, 999];
        let after = initial
            .into_iter()
            .flat_map(crate::replace_stone)
            .collect_vec();
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
        let simulate = input.clone().pipe(process);
        let simulate_no_alloc = input.clone().simulate_no_alloc(25);
        let sum = input.pipe(process_sum);
        assert_eq!(simulate, 218_079);
        assert_eq!(simulate_no_alloc, 218_079);
        assert_eq!(sum, 218_079);
        Ok(())
    }
}
