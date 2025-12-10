use color_eyre::{
    eyre::{bail, eyre, OptionExt},
    Result,
};
use nalgebra::ArrayStorage;
use tap::prelude::*;

use crate::{Machine, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(Puzzle { machines }: Puzzle) -> u64 {
    machines
        .iter()
        .map(|machine| fewest_presses(machine).expect("There to be a solution"))
        .sum()
}

fn fewest_presses(machine: &Machine) -> Result<u64> {
    let goal: Vector14 =
        from_goal(&machine.joltages).map_err(|n| eyre!("got {n} elements > 14"))?;
    let buttons: Matrix14 = from_buttons(&machine.buttons, machine.joltages.len())?;
    let plan = buttons.try_inverse().ok_or_eyre("Matrix not invertable")? * goal;
    plan.into_iter()
        .map(|&f| {
            if f.trunc() == f && f >= 0.0 {
                Ok(f as u64)
            } else {
                bail!("Fractional solution {f}")
            }
        })
        .try_fold(0, |sum, res| Ok(sum + res?))
}

type Vector14 = nalgebra::Vector<f32, nalgebra::U14, ArrayStorage<f32, 14, 1>>;
type Matrix14 = nalgebra::SquareMatrix<f32, nalgebra::U14, ArrayStorage<f32, 14, 14>>;

fn from_goal(joltages: &[u16]) -> Result<Vector14, usize> {
    if joltages.len() > 14 {
        Err(joltages.len())
    } else {
        joltages
            .iter()
            .rev()
            .copied()
            .map(f32::from)
            .chain(std::iter::repeat(0.0))
            .take(14)
            .pipe(Vector14::from_iterator)
            .pipe(Ok)
    }
}
// fn from_button(button: u16, len: usize) -> Result<Vector14> {
//     (0..len)
//         .map(move |shift| (button >> (len - shift - 1)) & 1)
//         .chain(std::iter::repeat(0))
//         .take(14)
//         .pipe(Vector14::from_iterator)
//         .pipe(Ok)
// }

fn from_buttons(buttons: &[u16], len: usize) -> Result<Matrix14> {
    if buttons.len() > 14 {
        bail!("Too many buttons ({})", buttons.len())
    }
    buttons
        .iter()
        .copied()
        .chain(std::iter::repeat(0))
        .take(14)
        .flat_map(move |button| {
            (0..len)
                .map(move |shift| (button >> (len - shift - 1)) & 1)
                .chain(std::iter::repeat(0))
                .take(14)
        })
        .map(f32::from)
        .pipe(Matrix14::from_iterator)
        .pipe(Ok)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    // #[rstest]
    // #[case::example("example.txt", 33)]
    // #[case::puzzle("input.txt", 0)]
    // fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
    //     let input: Puzzle = common::read_input!(input_path).parse()?;
    //     let output = process(input);
    //     assert_eq!(output, expected);
    //     Ok(())
    // }

    #[rstest]
    #[case("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", 10)]
    // #[case("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}", 12)]
    // #[case("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}", 11)]
    fn calculates_presses(#[case] machine: Machine, #[case] expected: u64) -> Result<()> {
        assert_eq!(fewest_presses(&machine)?, expected);
        Ok(())
    }
}
