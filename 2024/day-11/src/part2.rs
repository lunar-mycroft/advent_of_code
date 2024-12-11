use std::collections::HashMap;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> usize {
    let mut cache = HashMap::new();
    puzzle
        .stones
        .into_iter()
        // .take(1)
        .map(|stone| stones_after(stone, 75, &mut cache))
        .sum()
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
                    let s = n.to_string();
                    let (a, b) = s.split_at(s.len() / 2);

                    stones_after(a.parse().expect("will always be valid"), steps - 1, cache)
                        + stones_after(b.parse().expect("will always be valid"), steps - 1, cache)
                }
                n => stones_after(n * 2024, steps - 1, cache),
            };
            cache.insert((initial, steps), value);
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use super::*;

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part2.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 259_755_538_429_618);
        Ok(())
    }
}
