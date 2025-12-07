use glam::IVec2;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(mut puzzle: Puzzle) -> usize {
    puzzle.grid[IVec2::new(puzzle.start, 1)] = b'|';
    let mut res = 0;
    for y in 1..puzzle.grid.size().y - 1 {
        for x in 0..puzzle.grid.size().x {
            let pos = IVec2::new(x, y);
            if puzzle.grid[pos] != b'|' {
                continue;
            }
            let new_pos = pos + IVec2::Y;
            match puzzle.grid[new_pos] {
                b'.' => puzzle.grid[new_pos] = b'|',
                b'^' => {
                    res += 1;
                    puzzle.grid[new_pos + IVec2::X] = b'|';
                    puzzle.grid[new_pos - IVec2::X] = b'|';
                }
                b'|' => (),
                _ => unreachable!(),
            }
        }
    }
    res
}

// #[cfg(test)]
// mod tests {
//     use color_eyre::eyre::Result;
//     use rstest::rstest;

//     use super::*;

//     #[rstest]
//     #[case::example("example.txt", 21)]
//     #[case::part1("part1.txt", 1672)]
//     fn finds_solution(#[case] input_path: &str, #[case] expected: usize) -> Result<()> {
//         let input: Puzzle = common::read_input!(input_path).parse()?;
//         let output = process(input);
//         assert_eq!(output, expected);
//         Ok(())
//     }
// }
