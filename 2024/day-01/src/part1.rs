use std::num::ParseIntError;

use color_eyre::eyre::Result;
use tap::prelude::*;

pub fn process(input: &str) -> Result<String> {
    let (mut left, mut right) = input
        .split('\n')
        .map(str::trim)
        .filter_map(|line| line.split_once(' '))
        .map(|(first, second)| {
            (first.trim().parse::<u32>()?, second.trim().parse::<u32>()?).pipe(Ok)
        })
        .try_fold((Vec::new(), Vec::new()), |mut acc, res| {
            let (left, right) = res?;
            acc.0.push(left);
            acc.1.push(right);
            Ok::<_, ParseIntError>(acc)
        })?;
    left.sort_unstable();
    right.sort_unstable();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .sum::<u32>()
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
        assert_eq!(output, "11");
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part1.txt");
        let output = process(&input)?;
        assert_eq!(output, "1938424");
        Ok(())
    }
}
