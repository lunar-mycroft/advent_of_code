use std::str::FromStr;

use color_eyre::eyre::{OptionExt, Result};
use itertools::Itertools;
use tap::prelude::*;

pub fn process(input: &str) -> Result<String> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(safe)
        .filter_ok(|b| *b)
        .try_collect::<_, Vec<_>, _>()?
        .len()
        .to_string()
        .pipe(Ok)
}

fn safe(line: &str) -> Result<bool> {
    let nums: Vec<_> = line.split_whitespace().map(u32::from_str).try_collect()?;
    let monotonic = nums.iter().copied().tuple_windows().all(|(l, r)| l > r)
        || nums.iter().copied().tuple_windows().all(|(l, r)| l < r);
    let diff = nums
        .iter()
        .copied()
        .tuple_windows()
        .map(|(l, r)| l.abs_diff(r))
        .max()
        .ok_or_eyre("Empty row")?;
    Ok(monotonic && (1..=3).contains(&diff))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = common::read_input!("example.txt");
        let output = process(&input)?;
        assert_eq!(output, "2");
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part1.txt");
        let output = process(&input)?;
        assert_eq!(output, ("606"));
        Ok(())
    }
}
