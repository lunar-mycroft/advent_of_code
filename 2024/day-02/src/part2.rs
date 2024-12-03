use std::{collections::HashMap, str::FromStr};

use color_eyre::eyre::{OptionExt as _, Result};
use itertools::Itertools;
use tap::prelude::*;

pub fn process(input: &str) -> Result<String> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(safe_line)
        .filter_ok(|b| *b)
        .try_collect::<_, Vec<_>, _>()?
        .len()
        .to_string()
        .pipe(Ok)
}

fn safe_line(line: &str) -> Result<bool> {
    let nums: Vec<_> = line.split_whitespace().map(u32::from_str).try_collect()?;
    let Some(common_direction) = nums
        .iter()
        .tuple_windows()
        .map(|(l, r)| l.cmp(r))
        .fold(HashMap::new(), |mut map, ord| {
            *map.entry(ord).or_insert(0u32) += 1;
            map
        })
        .pipe(|map| {
            if map.len() == 1 {
                map.into_keys().next()
            } else if map.len() == 2 && map.values().copied().min().expect("2>0") == 1 {
                map.into_iter().max_by_key(|(_, v)| *v).map(|(o, _)| o)
            } else {
                None
            }
        })
    else {
        return Ok(false);
    };
    let bad_indexs = nums
        .iter()
        .copied()
        .enumerate()
        .tuple_windows()
        .flat_map(|((i, a), (j, b))| {
            if a.cmp(&b) == common_direction && (1..=3).contains(&a.abs_diff(b)) {
                vec![]
            } else {
                vec![i, j]
            }
        })
        .unique()
        .collect_vec();
    for idx in bad_indexs.iter().copied() {
        let nums = nums
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(i, n)| (i != idx).then_some(n))
            .collect_vec();
        let monotonic = nums.iter().copied().tuple_windows().all(|(l, r)| l > r)
            || nums.iter().copied().tuple_windows().all(|(l, r)| l < r);
        let diff = nums
            .iter()
            .copied()
            .tuple_windows()
            .map(|(l, r)| l.abs_diff(r))
            .max()
            .ok_or_eyre("Empty row")?;
        if monotonic && (1..=3).contains(&diff) {
            return Ok(true);
        }
    }
    bad_indexs.is_empty().pipe(Ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = common::read_input!("example.txt");
        let output = process(&input)?;
        assert_eq!(output, "4");
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part2.txt");
        let output = process(&input)?;
        assert_eq!(output, "644");
        Ok(())
    }
}
