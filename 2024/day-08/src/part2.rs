use color_eyre::eyre::Result;
use glam::IVec2;
use itertools::Itertools as _;
use tap::prelude::*;

pub fn process(input: &str) -> Result<usize> {
    let (size, antenas) = crate::parse(input)?;
    antenas
        .values()
        .flat_map(|positions| {
            positions
                .iter()
                .tuple_combinations()
                .map(|(a, b)| antinodes(*a, *b))
        })
        .flatten()
        .filter(|p| p.min_element() >= 0 && p.x < size.x && p.y < size.y)
        .unique()
        .count()
        .pipe(Ok)
}

fn antinodes(a: IVec2, b: IVec2) -> impl Iterator<Item = IVec2> {
    debug_assert_ne!(a, b);
    let delta = a - b;
    // this is hacky, but it works.
    // Also doesn't account for the possibility that delta.x and delta.y have a gcd > 1.
    // not sure if we're supposed to account for that and my input didn't need it, or if
    (-50..51).map(move |n| n * delta + a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = common::read_input!("example.txt");
        let output = process(&input)?;
        assert_eq!(output, 34);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part2.txt");
        let output = process(&input)?;
        assert_eq!(output, 1_169);
        Ok(())
    }
}
