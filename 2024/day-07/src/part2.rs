use color_eyre::eyre::Result;
use itertools::Itertools;
use rayon::prelude::*;
use tap::prelude::*;

use crate::Equation;

pub fn process(input: &str) -> Result<u64> {
    let equations: Vec<_> = input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::parse::<Equation>)
        .try_collect()?;
    equations
        .iter()
        .filter_map(|eq| can_match(eq.expr[0], &eq.expr[1..], eq.value).then_some(eq.value))
        .sum::<u64>()
        .pipe(Ok)
}

pub fn process_rayon(input: &str) -> Result<u64> {
    let equations: Vec<_> = input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::parse::<Equation>)
        .try_collect()?;
    equations
        .par_iter()
        .filter_map(|eq| can_match(eq.expr[0], &eq.expr[1..], eq.value).then_some(eq.value))
        .sum::<u64>()
        .pipe(Ok)
}

fn can_match(first: u64, rest: &[u64], expected: u64) -> bool {
    match rest {
        [second] => {
            first * (*second) == expected
                || first + (*second) == expected
                || concat(first, *second) == expected
        }
        [second, rest @ ..] => {
            let (mul, add, cat) = (first * (*second), first + (*second), concat(first, *second));
            can_match(mul, rest, expected)
                || can_match(add, rest, expected) | can_match(cat, rest, expected)
        }
        _ => unreachable!(),
    }
}

const fn concat(left: u64, right: u64) -> u64 {
    (10u64).pow(right.ilog(10) + 1) * left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        assert_eq!(concat(12, 34), 1234);
    }

    #[test]
    fn test_example() -> Result<()> {
        let input = common::read_input!("example.txt");
        let output = process(&input)?;
        assert_eq!(output, 11_387);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part2.txt");
        let output = process(&input)?;
        assert_eq!(output, 492_383_931_650_959);
        Ok(())
    }
}
