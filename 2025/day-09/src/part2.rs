use std::ops::RangeInclusive;

use glam::IVec2;
use itertools::Itertools;
use tap::prelude::*;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(Puzzle { tiles }: Puzzle) -> u64 {
    let segments = tiles
        .iter()
        .copied()
        .tuple_windows::<(_, _)>()
        .chain(
            (
                *tiles.last().expect("Known non-empty"),
                *tiles.first().expect("Known non-empty"),
            )
                .pipe(std::iter::once),
        )
        .collect_vec();
    tiles
        .iter()
        .copied()
        .tuple_combinations::<(_, _)>()
        .filter(|&rect| !segments.iter().copied().any(|seg| intersects(rect, seg)))
        .map(|(a, b)| crate::area(a, b))
        .max()
        .expect("Knonw non-empty")
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
        panic!("Diagonal segment")
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 24)]
    #[case::puzzle("input.txt", 0)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }

    #[rstest]
    #[case((IVec2::new(11,7), IVec2::new(2,3)), (IVec2::new(9,5), IVec2::new(9,7)), true)]
    fn intersections_correct(
        #[case] rect: (IVec2, IVec2),
        #[case] seg: (IVec2, IVec2),
        #[case] expected: bool,
    ) {
        assert_eq!(intersects(rect, seg), expected);
    }

    #[rstest]
    #[case(IVec2::new(9, 5), IVec2::new(2, 3), true)]
    #[case(IVec2::new(9, 7), IVec2::new(2, 3), false)]
    #[case(IVec2::new(2, 5), IVec2::new(11, 1), false)]
    fn solutions_correct(#[case] a: IVec2, #[case] b: IVec2, #[case] valid: bool) -> Result<()> {
        let Puzzle { tiles } = common::read_input!("example.txt").parse()?;
        let segments = tiles
            .iter()
            .copied()
            .tuple_windows::<(_, _)>()
            .chain(
                (
                    *tiles.last().expect("Known non-empty"),
                    *tiles.first().expect("Known non-empty"),
                )
                    .pipe(std::iter::once),
            )
            .collect_vec();
        let rect = (a.min(b), a.max(b));

        let violation = segments.into_iter().find(|&seg| intersects(rect, seg));
        match violation {
            Some((c, d)) if valid => panic!("segment {c}-{d} incorrectly says {a}:{b} is invalid"),
            None if !valid => panic!("{a}:{b} is invalid"),
            _ => Ok(()),
        }
    }
}
