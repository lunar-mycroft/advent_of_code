use std::collections::HashSet;

use crate::Puzzle;

fn count_visited(costs: &crate::Costs, puzzle: &Puzzle) -> usize {
    let end_dir = crate::DIRECTIONS
        .iter()
        .copied()
        .min_by_key(|d| costs.get(puzzle.end, *d).unwrap_or(u32::MAX))
        .expect("a solution to be found");
    let mut stack = vec![(puzzle.end, end_dir)];
    let mut seen = HashSet::new();
    while let Some((pos, dir)) = stack.pop() {
        seen.insert(pos);
        let current_cost = costs.get(pos, dir).expect("");
        if current_cost == 0 {
            continue;
        }
        let back = (pos - dir, dir);

        let t1 = (pos, dir.perp());
        let t2 = (pos, -dir.perp());
        match costs.get(back.0, back.1) {
            Some(cost) if (cost + 1) == current_cost => {
                stack.push(back);
            }
            _ => (),
        }
        match costs.get(t1.0, t1.1) {
            Some(cost) if (cost + 1000) == current_cost => stack.push(t1),
            _ => (),
        }
        match costs.get(t2.0, t2.1) {
            Some(cost) if (cost + 1000) == current_cost => stack.push(t2),
            _ => (),
        }
    }
    seen.len()
}

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn bfs(puzzle: Puzzle) -> usize {
    let costs = puzzle.bfs();
    count_visited(&costs, &puzzle)
}

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn dijkstras(puzzle: Puzzle) -> usize {
    let costs = puzzle.dijkstras();

    count_visited(&costs, &puzzle)
}

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn astar(puzzle: Puzzle) -> usize {
    let costs = puzzle.astar();

    count_visited(&costs, &puzzle)
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
        assert_eq!(b, 45);
        assert_eq!(d, 45);
        assert_eq!(a, 45);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part2.txt").parse()?;
        let b = input.clone().pipe(bfs);
        let d = input.clone().pipe(dijkstras);
        let a = input.pipe(astar);
        assert_eq!(b, 489);
        assert_eq!(d, 489);
        assert_eq!(a, 489);
        Ok(())
    }
}
