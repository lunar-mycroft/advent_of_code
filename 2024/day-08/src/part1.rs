use color_eyre::eyre::Result;
use glam::IVec2;
use itertools::Itertools;
use tap::prelude::*;

pub fn process(input: &str) -> Result<usize> {
    let (size, antenas) = crate::parse(input)?;

    antenas
        .values()
        .flat_map(|positions| {
            positions
                .iter()
                .tuple_combinations()
                .map(|(a, b)| antinodes(*a, *b).conv::<[_; 2]>())
        })
        .flatten()
        .filter(|p| p.min_element() >= 0 && p.x < size.x && p.y < size.y)
        .unique()
        .count()
        .pipe(Ok)
}

fn antinodes(a: IVec2, b: IVec2) -> (IVec2, IVec2) {
    debug_assert_ne!(a, b);
    let delta = a - b;
    (a + delta, b - delta)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = common::read_input!("example.txt");
        let output = process(&input)?;
        assert_eq!(output, 14);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part1.txt");
        let output = process(&input)?;
        assert_eq!(output, 369);
        Ok(())
    }
}
