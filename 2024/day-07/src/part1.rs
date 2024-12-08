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

/*
initially tried checking in reverse order, which failed.  Had to use posted solutions as
oracle to debug, noticed the working solutions matched forward.  Optimized by avoiding
allocating a new array
*/
fn can_match(first: u64, rest: &[u64], expected: u64) -> bool {
    match rest {
        [second] => first * (*second) == expected || first + (*second) == expected,
        [second, rest @ ..] => {
            let (mul, add) = (first * (*second), first + (*second));
            can_match(mul, rest, expected) || can_match(add, rest, expected)
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = common::read_input!("example.txt");
        let output = process(&input)?;
        assert_eq!(output, 3749);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part1.txt");
        let output = process(&input)?;
        assert_eq!(output, 5_837_374_519_342);
        Ok(())
    }
}
