use std::mem::swap;

use fxhash::FxHashMap;
use tap::prelude::*;

use crate::{Gate, Operation, Puzzle, Wire};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: &Puzzle) -> Option<String> {
    let output_of: FxHashMap<_, _> = puzzle
        .operations
        .iter()
        .flat_map(|(key, val)| [(*val, *key), (val.reveresed(), *key)])
        .collect();
    let mut swapped = Vec::new();
    let mut c = (None, None);
    for b in 0..45 {
        c.1 = None;
        let (mut r1, mut z1) = (None, None);

        // Half adder logic
        let mut m1 = output_of
            .get(&Gate {
                left: Wire::X(b),
                op: Operation::Xor,
                right: Wire::Y(b),
            })
            .copied();
        let mut n1 = output_of
            .get(&Gate {
                left: Wire::X(b),
                op: Operation::And,
                right: Wire::Y(b),
            })
            .copied();
        if let Some(c0) = c.0 {
            r1 = output_of
                .get(&Gate {
                    left: c0,
                    op: Operation::And,
                    right: m1?,
                })
                .copied();
            if r1.is_none() {
                swap(&mut m1, &mut n1);
                swapped.extend([puzzle.wire_str(m1?), puzzle.wire_str(n1?)]);
                r1 = output_of
                    .get(&Gate {
                        left: c0,
                        op: Operation::And,
                        right: m1?,
                    })
                    .copied();
            }

            z1 = output_of
                .get(&Gate {
                    left: c0,
                    op: Operation::Xor,
                    right: m1?,
                })
                .copied();

            if let Some(Wire::Z(_)) = m1 {
                swap(&mut m1, &mut z1);
                // swapped.extend([m1?, z1?]);
                swapped.extend([puzzle.wire_str(m1?), puzzle.wire_str(z1?)]);
            }

            if let Some(Wire::Z(_)) = n1 {
                swap(&mut n1, &mut z1);
                // swapped.extend([n1?, z1?]);
                swapped.extend([puzzle.wire_str(n1?), puzzle.wire_str(z1?)]);
            }

            if let Some(Wire::Z(_)) = r1 {
                swap(&mut r1, &mut z1);
                // swapped.extend([r1?, z1?]);
                swapped.extend([puzzle.wire_str(r1?), puzzle.wire_str(z1?)]);
            }

            // c.1 = puzzle.output_of(r1?, "OR", n1?);
            c.1 = output_of
                .get(&Gate {
                    left: r1?,
                    op: Operation::Or,
                    right: n1?,
                })
                .copied();
        }

        match c.1 {
            Some(Wire::Z(45) | Wire::X(_) | Wire::Y(_) | Wire::Other(_)) | None => (),
            Some(Wire::Z(_)) => {
                swap(&mut c.1, &mut z1);
                // swapped.extend([c.1?, z1?]);
                swapped.extend([puzzle.wire_str(c.1?), puzzle.wire_str(z1?)]);
            }
        }

        // c0 = c1 if c0 else n1
        c.0 = if c.0.is_some() { c.1 } else { n1 };
    }
    debug_assert_eq!(swapped.len(), 8);
    swapped.sort_unstable();
    swapped.join(",").pipe(Some)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("part2.txt", "btb,cmv,mwp,rdg,rmj,z17,z23,z30")]
    fn finds_solution(#[case] input_path: &str, #[case] expected: &str) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(&input);
        assert_eq!(output.expect("to find a solution"), expected);
        Ok(())
    }
}
