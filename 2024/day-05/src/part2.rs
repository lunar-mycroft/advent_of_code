use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    str::FromStr as _,
};

use color_eyre::eyre::{OptionExt as _, Result};
use itertools::Itertools as _;
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
                .try_collect::<_, HashSet<_>, color_eyre::Report>()?,
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
        .map(|p| correct_order(&ordering_rules, p))
        .sum::<u32>()
        .pipe(Ok)
}

fn correct_order(rules: &HashSet<(u32, u32)>, pages: &[u32]) -> u32 {
    if in_correct_order(rules, pages) {
        0
    } else {
        let mut pages = pages.to_vec();
        pages.sort_by(|a, b| {
            if rules.contains(&(*a, *b)) {
                Ordering::Less
            } else if rules.contains(&(*b, *a)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        middle_page(&pages)
    }
}

fn in_correct_order(rules: &HashSet<(u32, u32)>, pages: &[u32]) -> bool {
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
        assert_eq!(output, 123);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part2.txt");
        let output = process(&input)?;
        assert_ne!(output, 5068);
        Ok(())
    }
}
