use std::mem::swap;

use tap::prelude::*;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: &Puzzle) -> Option<String> {
    let mut swapped = Vec::new();
    let mut c = (None, None);
    for b in 0..45 {
        let n = format!("{b:0>2}");
        c.1 = None;
        let (mut r1, mut z1) = (None, None);

        // Half adder logic
        let mut m1 = puzzle.output_of(&format!("x{n}"), "XOR", &format!("y{n}"));
        let mut n1 = puzzle.output_of(&format!("x{n}"), "AND", &format!("y{n}"));
        if let Some(c0) = c.0 {
            // r1 = find(c0, m1, "AND", data[1])
            r1 = puzzle.output_of(c0, "AND", m1?);
            if r1.is_none() {
                swap(&mut m1, &mut n1);
                swapped.extend([m1?, n1?]);
                r1 = puzzle.output_of(c0, "AND", m1?);
            }

            z1 = puzzle.output_of(c0, "XOR", m1?);

            if m1.is_some_and(|m| m.starts_with('z')) {
                swap(&mut m1, &mut z1);
                swapped.extend([m1?, z1?]);
            }

            if n1.is_some_and(|m| m.starts_with('z')) {
                swap(&mut n1, &mut z1);
                swapped.extend([n1?, z1?]);
            }

            if r1.is_some_and(|m| m.starts_with('z')) {
                swap(&mut r1, &mut z1);
                swapped.extend([r1?, z1?]);
            }

            c.1 = puzzle.output_of(r1?, "OR", n1?);
        }

        if c.1.is_some_and(|c| c.starts_with('z') && c != "z45") {
            swap(&mut c.1, &mut z1);
            swapped.extend([c.1?, z1?]);
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
