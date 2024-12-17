use itertools::Itertools;

use crate::Puzzle;

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(puzzle: Puzzle) -> String {
    puzzle
        .iter()
        .take(1_000_000)
        .map(|n| n.to_string())
        .join(",")
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use super::*;

    #[test]
    fn test_small() {
        {
            let p = Puzzle {
                a: 0,
                b: 0,
                c: 9,
                program: vec![2, 6],
            };
            let mut cpu = p.iter();

            assert_eq!(cpu.run(), vec![]);
            assert_eq!(cpu.b, 1);
        }
        {
            let p = Puzzle {
                a: 10,
                b: 0,
                c: 0,
                program: vec![5, 0, 5, 1, 5, 4],
            };
            let mut cpu = p.iter();

            assert_eq!(cpu.run(), vec![0, 1, 2]);
        }
        {
            let p = Puzzle {
                a: 2024,
                b: 0,
                c: 0,
                program: vec![0, 1, 5, 4, 3, 0],
            };
            let mut cpu = p.iter();
            assert_eq!(cpu.run(), vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
            assert_eq!(cpu.a, 0);
        }
        {
            let p = Puzzle {
                a: 0,
                b: 29,
                c: 0,
                program: vec![1, 7],
            };
            let mut cpu = p.iter();
            assert_eq!(cpu.run(), vec![]);
            assert_eq!(cpu.b, 26);
        }
        {
            let p = Puzzle {
                a: 0,
                b: 2024,
                c: 43690,
                program: vec![4, 0],
            };
            let mut cpu = p.iter();
            assert_eq!(cpu.run(), vec![]);
            assert_eq!(cpu.b, 44354);
        }
    }

    #[test]
    fn test_example() -> Result<()> {
        let input: Puzzle = common::read_input!("example.txt").parse()?;
        let output = process(input);
        assert_eq!(output, "4,6,3,5,6,3,5,2,1,0");
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input: Puzzle = common::read_input!("part1.txt").parse()?;
        let output = process(input);
        assert_eq!(output, "2,7,4,7,2,1,7,5,1");
        Ok(())
    }
}
