use std::{collections::HashMap, num::ParseIntError};

use color_eyre::eyre::Result;
use tap::prelude::*;

pub fn process(input: &str) -> Result<String> {
    let (left, counts) = input
        .split('\n')
        .map(str::trim)
        .filter_map(|line| line.split_once(' '))
        .map(|(first, second)| {
            (first.trim().parse::<u64>()?, second.trim().parse::<u64>()?).pipe(Ok)
        })
        .try_fold((Vec::new(), HashMap::new()), |(mut v, mut c), res| {
            let (left, right) = res?;
            v.push(left);
            *c.entry(right).or_insert(0) += 1u64;
            Ok::<_, ParseIntError>((v, c))
        })?;
    left.into_iter()
        .map(|n| n * counts.get(&n).copied().unwrap_or(0))
        .sum::<u64>()
        .to_string()
        .pipe(Ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = common::read_input!("example.txt");
        let output = process(&input)?;
        assert_eq!(output, "31");
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part1.txt");
        let output = process(&input)?;
        assert_eq!(output, "22014209");
        Ok(())
    }
}
