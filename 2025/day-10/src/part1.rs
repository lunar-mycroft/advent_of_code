use crate::{Machine, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(Puzzle { machines }: Puzzle) -> u64 {
    machines
        .iter()
        .map(|machine| fewest_presses(machine).expect("There to be a solution"))
        .sum()
}

fn fewest_presses(machine: &Machine) -> Option<u64> {
    (1u16..(1 << machine.buttons.len()))
        .filter(|&activation| {
            let actual = machine
                .buttons
                .iter()
                .copied()
                .enumerate()
                .filter_map(|(shift, button)| {
                    if ((1 << shift) & activation) != 0 {
                        Some(button)
                    } else {
                        None
                    }
                })
                .fold(0, |lights, button| lights ^ button);
            machine.goal == actual
        })
        .map(u16::count_ones)
        .min()
        .map(u64::from)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 7)]
    #[case::puzzle("input.txt", 428)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }

    #[rstest]
    #[case("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", 2)]
    #[case("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}", 3)]
    #[case("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}", 2)]
    #[case("[##.##.....] (1,2,3,4,9) (0,7) (0,3,8,9) (0,2) (0,1,8) (1,2,9) (0,2,3,5,6,7,8) (0,1,3,4,6,7) (7) (0,2,3,4,5,6,7,8) (0,1,3,7,8,9) (0,1,2,3,4,5,8,9) (0,1,3,4,6,7,9) {79,44,43,71,37,21,28,61,55,60}", 4)]
    fn calculates_presses(#[case] machine: Machine, #[case] expected: u64) {
        assert_eq!(fewest_presses(&machine), Some(expected));
    }
}
