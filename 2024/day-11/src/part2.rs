use std::collections::HashMap;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> usize {
    #[allow(clippy::option_if_let_else)]
    fn stones_after(initial: u64, steps: u8, cache: &mut HashMap<(u64, u8), usize>) -> usize {
        match cache.get(&(initial, steps)) {
            Some(value) => *value,
            None => {
                let value = match initial {
                    _ if steps == 0 => 1,
                    0 => stones_after(1, steps - 1, cache),
                    n if n.ilog(10) % 2 == 1 => {
                        let s = n.to_string();
                        let (a, b) = s.split_at(s.len() / 2);

                        stones_after(a.parse().expect("will always be valid"), steps - 1, cache)
                            + stones_after(
                                b.parse().expect("will always be valid"),
                                steps - 1,
                                cache,
                            )
                    }
                    n => stones_after(n * 2024, steps - 1, cache),
                };
                cache.insert((initial, steps), value);
                value
            }
        }
    }

    let mut cache = HashMap::new();
    puzzle.sum(75, move |stone, steps| {
        stones_after(stone, steps, &mut cache)
    })
}

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process_str_free(puzzle: Puzzle) -> usize {
    const fn split_digits(stone: u64) -> (u64, u64) {
        let div = 10u64.pow(stone.ilog10() / 2 + 1);
        (stone / div, stone % div)
    }
    #[allow(clippy::option_if_let_else)]
    fn stones_after(initial: u64, steps: u8, cache: &mut HashMap<(u64, u8), usize>) -> usize {
        match cache.get(&(initial, steps)) {
            Some(value) => *value,
            None => {
                let value = match initial {
                    _ if steps == 0 => 1,
                    0 => stones_after(1, steps - 1, cache),
                    n if n.ilog(10) % 2 == 1 => {
                        let (a, b) = split_digits(n);
                        stones_after(a, steps - 1, cache) + stones_after(b, steps - 1, cache)
                    }
                    n => stones_after(n * 2024, steps - 1, cache),
                };
                cache.insert((initial, steps), value);
                value
            }
        }
    }

    let mut cache = HashMap::new();
    puzzle.sum(75, move |stone, steps| {
        stones_after(stone, steps, &mut cache)
    })
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use tap::prelude::*;

    use super::*;

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part2.txt").parse()?;
        let naive = input.clone().pipe(process);
        let str_free = input.pipe(process_str_free);
        assert_eq!(naive, 259_755_538_429_618);
        assert_eq!(str_free, 259_755_538_429_618);
        Ok(())
    }
}
