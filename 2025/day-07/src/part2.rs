use common::grid::Grid;
use glam::IVec2;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> usize {
    let mut counts: Grid<usize> = Grid::from_value(0, puzzle.grid.size());
    counts[IVec2::new(puzzle.start, 1)] = 1;

    let mut paths_in_row = 0;
    for y in 1..puzzle.grid.size().y - 1 {
        paths_in_row = 0;
        for x in 0..puzzle.grid.size().x {
            let pos = IVec2::new(x, y);
            let paths = counts[pos];
            if paths == 0 {
                continue;
            }
            let new_pos = pos + IVec2::Y;
            match puzzle.grid[new_pos] {
                b'.' => {
                    paths_in_row += paths;
                    counts[new_pos] += paths;
                }
                b'^' => {
                    counts[new_pos + IVec2::X] += paths;
                    counts[new_pos - IVec2::X] += paths;
                    paths_in_row += paths * 2;
                }
                _ => unreachable!(),
            }
        }
    }
    paths_in_row
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 40)]
    #[case::part2("part2.txt", 231_229_866_702_355)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: usize) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }
}
