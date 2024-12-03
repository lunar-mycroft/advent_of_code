use color_eyre::eyre::Result;
use tap::prelude::*;

pub fn process(mut input: &str) -> Result<u32> {
    std::iter::from_fn(|| loop {
        match input.split_at(input.find("mul(")?).1.pipe(parse_mul) {
            Ok((l, r, rest)) => {
                input = rest;
                break (l, r).pipe(Some);
            }
            Err("") => break None,
            Err(rest) => {
                input = rest;
                continue;
            }
        }
    })
    .map(|(l, r)| l * r)
    .sum::<u32>()
    .pipe(Ok)
}

fn parse_mul(s: &str) -> Result<(u32, u32, &str), &str> {
    let nums = s.strip_prefix("mul(").ok_or(s)?;
    let (left, rest) = parse_num(nums)?;
    let (right, rest) = rest.strip_prefix(',').ok_or(rest)?.pipe(parse_num)?;
    if rest.starts_with(')') {
        (left, right, rest).pipe(Ok)
    } else {
        Err(rest)
    }
}

fn parse_num(s: &str) -> Result<(u32, &str), &str> {
    let idx = s.find(|c: char| !c.is_ascii_digit()).ok_or(s)?;
    let (n, rest) = s.split_at(idx);
    if rest.starts_with(|c: char| [',', ')'].contains(&c)) {
        let n = n.parse::<u32>().map_err(|_| rest)?;
        (n, rest).pipe(Ok)
    } else {
        Err(rest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let output = process(input)?;
        assert_eq!(output, 161);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part1.txt");
        let output = process(&input)?;
        assert_eq!(output, 173_731_097);
        Ok(())
    }
}
