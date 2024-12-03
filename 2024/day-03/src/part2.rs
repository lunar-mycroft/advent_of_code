use color_eyre::eyre::Result;
use tap::prelude::*;

pub fn process(mut input: &str) -> Result<String> {
    let mut sum = 0u32;
    while !input.is_empty() {
        let (prefix, suffix) = input.split_once("don't()").unwrap_or((input, ""));
        sum += super::part1::process(prefix)?;
        let (_, suffix) = suffix.split_once("do()").unwrap_or((suffix, ""));
        input = suffix;
    }
    sum.to_string().pipe(Ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let output = process(input)?;
        assert_eq!(output, "48");
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part2.txt");
        let output = process(&input)?;
        assert_eq!(output, "93729253");
        Ok(())
    }
}
