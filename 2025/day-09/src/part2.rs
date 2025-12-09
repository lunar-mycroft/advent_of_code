use glam::IVec2;
use itertools::Itertools;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(Puzzle { tiles }: Puzzle) -> Option<u64> {
    let segments = tiles
        .iter()
        .copied()
        .circular_tuple_windows::<(_, _)>()
        .collect_vec();

    {
        let mut by_area = tiles
            .iter()
            .copied()
            .tuple_combinations::<(_, _)>()
            .map(|(a, b)| (a, b, crate::area(a, b)))
            .collect_vec();
        by_area.sort_unstable_by(|&(_, _, a_0), (_, _, a_1)| a_0.cmp(a_1));
        by_area
    }
    .into_iter()
    .rev()
    .find_map(|(a, b, area)| {
        if segments.iter().copied().any(|seg| intersects((a, b), seg)) {
            None
        } else {
            Some(area)
        }
    })
}

fn intersects(rect: (IVec2, IVec2), seg: (IVec2, IVec2)) -> bool {
    let rect = (rect.0.min(rect.1), rect.0.max(rect.1));
    // Valid because segment is not diagonal
    let seg = (seg.0.min(seg.1), seg.0.max(seg.1));

    let left = rect.1.x <= seg.0.x;
    let right = rect.0.x >= seg.1.x;
    let above = rect.1.y <= seg.0.y;
    let below = rect.0.y >= seg.1.y;
    !(left || right || above || below)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 24)]
    #[case::puzzle("input.txt", 1_476_550_548)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, Some(expected));
        Ok(())
    }
}
