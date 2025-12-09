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
        by_area.sort_unstable_by_key(|&(_, _, area)| area);
        by_area
    }
    .into_iter()
    .rev()
    .find(|&(a, b, _)| !segments.iter().copied().any(|seg| intersects((a, b), seg)))
    .map(|(_, _, area)| area)
}

fn intersects(rect: (IVec2, IVec2), seg: (IVec2, IVec2)) -> bool {
    let rect = (rect.0.min(rect.1), rect.0.max(rect.1));
    if seg.0.x == seg.1.x {
        let x = seg.0.x;
        if !(rect.0.x < x && rect.1.x > x) {
            return false;
        }
        let (min, max) = (seg.0.y.min(seg.1.y), seg.0.y.max(seg.1.y));
        if max == rect.0.y || min == rect.1.y {
            return false;
        }
        let range = min..=max;
        range.contains(&rect.0.y) || range.contains(&rect.1.y)
    } else if seg.0.y == seg.1.y {
        let y = seg.0.y;
        if !(rect.0.y < y && rect.1.y > y) {
            return false;
        }
        let (min, max) = (seg.0.x.min(seg.1.x), seg.0.x.max(seg.1.x));
        if max == rect.0.x || min == rect.1.x {
            return false;
        }
        let range = min..=max;
        range.contains(&rect.0.x) || range.contains(&rect.1.x)
    } else {
        unreachable!()
    }
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
