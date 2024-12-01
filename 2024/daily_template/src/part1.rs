use color_eyre::eyre::Result;
use tap::prelude::*;

pub fn process(input: &str) -> Result<String> {
    todo!("{{crate_name}} part1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = common::read_input!("example.txt");
        let output = process(&input)?;
        assert_eq!(output, todo!("{{crate_name}} part1"));
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part1.txt");
        let output = process(&input)?;
        assert_eq!(output, todo!("{{crate_name}} part1"));
        Ok(())
    }
}
