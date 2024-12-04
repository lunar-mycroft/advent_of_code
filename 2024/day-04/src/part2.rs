use color_eyre::eyre::{ensure, Result};
use itertools::Itertools;
use tap::prelude::*;

pub fn process(input: &str) -> Result<usize> {
    let lines = input.lines().collect_vec();
    let width = lines[0].len();
    ensure!(width == lines.len(), "Non square inputs not allowed");
    (1..width - 1)
        .cartesian_product(1..width - 1)
        .map(|(x, y)| {
            (
                format!(
                    "{}{}{}",
                    get_char(&lines, (x - 1, y - 1)),
                    get_char(&lines, (x, y)),
                    get_char(&lines, (x + 1, y + 1))
                ),
                format!(
                    "{}{}{}",
                    get_char(&lines, (x - 1, y + 1)),
                    get_char(&lines, (x, y)),
                    get_char(&lines, (x + 1, y - 1))
                ),
            )
        })
        .filter(|(a, b)| match (a.as_str(), b.as_str()) {
            #[expect(clippy::unnested_or_patterns)]
            ("MAS", "MAS") | ("SAM", "SAM") | ("MAS", "SAM") | ("SAM", "MAS") => true,
            _ => false,
        })
        .count()
        .pipe(Ok)
}

fn get_char<'a>(lines: &'a [&'a str], (x, y): (usize, usize)) -> &'a str {
    &lines[y][x..=x]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = common::read_input!("example.txt");
        let output = process(&input)?;
        assert_eq!(output, 9);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part2.txt");
        let output = process(&input)?;
        assert_eq!(output, 1824);
        Ok(())
    }
}
