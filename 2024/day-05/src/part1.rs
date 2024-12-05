use color_eyre::eyre::{OptionExt, Result};
use itertools::Itertools;
use std::{collections::HashMap, str::FromStr as _};
use tap::prelude::*;

pub fn process(input: &str) -> Result<u32> {
    let (ordering_rules, pages) = {
        let cleaned = input.replace('\r', "");
        let (or_sec, p_sec) = cleaned
            .split_once("\n\n")
            .ok_or_eyre("Couldn't split_section")?;
        (
            or_sec
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let (a, b) = s.split_once('|').ok_or_eyre("Invalid ordering")?;
                    (a.parse::<u32>()?, b.parse::<u32>()?).pipe(Ok)
                })
                .try_collect::<_, Vec<_>, color_eyre::Report>()?,
            p_sec
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|s| {
                    s.split(',')
                        .map(u32::from_str)
                        .try_collect::<_, Vec<_>, _>()
                })
                .try_collect::<_, Vec<_>, _>()?,
        )
    };

    pages
        .iter()
        .filter(|p| in_correct_order(&ordering_rules, p))
        .map(|p| middle_page(p))
        .sum::<u32>()
        .pipe(Ok)
}

fn in_correct_order(rules: &[(u32, u32)], pages: &[u32]) -> bool {
    let order_map: HashMap<_, _> = pages
        .iter()
        .copied()
        .enumerate()
        .map(|(idx, p)| (p, idx))
        .collect();
    rules.iter().all(
        |(a, b)| match (order_map.get(a).copied(), order_map.get(b).copied()) {
            (Some(a_idx), Some(b_idx)) if (a_idx < b_idx) => true,
            (Some(_), Some(_)) => false,
            _ => true,
        },
    )
}

fn middle_page(pages: &[u32]) -> u32 {
    assert!(pages.len() % 2 != 0, "Even number of pages {}", pages.len());
    pages[pages.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = common::read_input!("example.txt");
        let output = process(&input)?;
        assert_eq!(output, 143);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part1.txt");
        let output = process(&input)?;
        assert_eq!(output, 7307);
        Ok(())
    }
}
