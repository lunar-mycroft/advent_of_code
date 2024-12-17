use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn bfs(puzzle: Puzzle) -> u32 {
    let costs = puzzle.bfs();

    crate::DIRECTIONS
        .iter()
        .copied()
        .filter_map(|d| costs.get(puzzle.end, d))
        .min()
        .expect("a solution to be found")
}

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn dijkstras(puzzle: Puzzle) -> u32 {
    let costs = puzzle.dijkstras();

    crate::DIRECTIONS
        .iter()
        .copied()
        .filter_map(|d| costs.get(puzzle.end, d))
        .min()
        .expect("a solution to be found")
}

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn astar(puzzle: Puzzle) -> u32 {
    let costs = puzzle.astar();

    crate::DIRECTIONS
        .iter()
        .copied()
        .filter_map(|d| costs.get(puzzle.end, d))
        .min()
        .expect("a solution to be found")
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use tap::prelude::*;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let b = input.clone().pipe(bfs);
        let d = input.clone().pipe(dijkstras);
        let a = input.pipe(astar);
        assert_eq!(b, 7036);
        assert_eq!(d, 7036);
        assert_eq!(a, 7036);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part1.txt").parse()?;
        let b = input.clone().pipe(bfs);
        let d = input.clone().pipe(dijkstras);
        let a = input.pipe(astar);
        assert_eq!(b, 65436);
        assert_eq!(d, 65436);
        assert_eq!(a, 65436);
        Ok(())
    }
}
