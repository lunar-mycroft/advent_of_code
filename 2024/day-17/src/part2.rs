use itertools::Itertools;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> u64 {
    // can only claim partial credit for this.  I noticed that the was a function of
    // the least significant bits of a, but didn't realize that the bits involved could overlap
    let mut factors = vec![0u64; puzzle.program.len()];
    *factors.last_mut().expect("Non empty") = 1;
    loop {
        let mut cpu = puzzle.clone();
        let a = factors
            .iter()
            .copied()
            .enumerate()
            .map(|(p, f)| f << ((p * 3) as u64))
            .sum::<u64>();
        cpu.a = a;
        let output = cpu.collect_vec();
        if output == puzzle.program {
            break a;
        }

        for idx in (0..puzzle.program.len()).rev() {
            if output[idx] != puzzle.program[idx] {
                factors[idx] += 1;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use super::*;

    #[test]
    fn test_example() {
        let input: Puzzle = Puzzle {
            a: 117_440,
            b: 0,
            c: 0,
            program_counter: 0,
            program: vec![0, 3, 5, 4, 3, 0],
        };
        assert!(!input
            .clone()
            .zip(input.program)
            .any(|(actual, expected)| actual != expected));
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part2.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 37_221_274_271_220);
        Ok(())
    }
}
