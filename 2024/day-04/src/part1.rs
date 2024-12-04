use color_eyre::eyre::{ensure, Result};
use itertools::Itertools;

pub fn process(input: &str) -> Result<usize> {
    ensure!(input.is_ascii(), "Non ascii input not allowed");
    let lines = input.lines().collect_vec();
    ensure!(
        lines[0].len() == lines.len(),
        "Non square inputs not allowed"
    );
    let width = lines[0].len();
    let vertical = (0..width)
        .map(|x| lines.iter().map(|line| &line[x..=x]).collect::<String>())
        .collect_vec();
    let down_right = (0..width)
        .map(|x| {
            lines
                .iter()
                .enumerate()
                .take(width - x)
                .map(|(y, line)| &line[(y + x)..=(y + x)])
                .collect::<String>()
        })
        .rev()
        .chain((1..width).map(|y| {
            (0..(width - y))
                .map(|x| (x, x + y))
                .map(|(x, y)| &lines[y][x..=x])
                .collect()
        }))
        .collect_vec();
    let down_left = (0..width)
        .map(|x| {
            lines
                .iter()
                .enumerate()
                .take(width - x)
                .map(|(y, line)| {
                    let idx = width - y - x - 1;
                    &line[idx..=idx]
                })
                .collect::<String>()
        })
        .rev()
        .chain((1..width).map(|y| {
            (0..(width - y))
                .map(|x| (width - x - 1, x + y))
                .map(|(x, y)| &lines[y][x..=x])
                .collect()
        }))
        .collect_vec();
    let h_sum = lines.iter().map(|line| count_in_line(line)).sum::<usize>();
    let v_sum = vertical
        .iter()
        .map(|line| count_in_line(line))
        .sum::<usize>();
    let d_sum = down_right
        .iter()
        .chain(&down_left)
        .map(|line| count_in_line(line))
        .sum::<usize>();
    Ok(v_sum + h_sum + d_sum)
}

fn count_in_line(line: &str) -> usize {
    line.split("XMAS").count() + line.rsplit("SAMX").count() - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = common::read_input!("example.txt");
        let output = process(&input)?;
        assert_eq!(output, 18);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part1.txt");
        let output = process(&input)?;
        assert_eq!(output, 2_397);
        Ok(())
    }
}
