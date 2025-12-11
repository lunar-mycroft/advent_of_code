use color_eyre::{eyre::OptionExt, Result};
use itertools::Itertools;
use tap::prelude::*;

use crate::{Machine, Puzzle};

#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn process(Puzzle { machines }: Puzzle) -> u64 {
    machines
        .iter()
        .map(|machine| fewest_presses(machine).expect("There to be a solution"))
        .sum()
}

// Based on /u/RussellDash's solution (found at
// https://www.reddit.com/r/adventofcode/comments/1pity70/2025_day_10_solutions/nt988z4/
// and https://github.com/RussellDash332/advent-of-code/blob/main/aoc-2025%2FDay-10%2FPython%2Fmain.py)
// as adapted by /u/Ok-Bus4754 (
// https://www.reddit.com/r/adventofcode/comments/1pity70/2025_day_10_solutions/nt9m1wg/
// and https://github.com/Fadi88/AoC/tree/master/2025/days/day10)
fn fewest_presses(machine: &Machine) -> Result<u64> {
    let (n_buttons, n_goals) = (machine.buttons.len(), machine.joltages.len());
    let (height, rows) = (2 * n_goals + n_buttons, n_buttons + 1);

    let mut matrix = vec![vec![0.0; rows]; height];
    for (i, row) in matrix.iter_mut().rev().take(n_buttons).enumerate() {
        row[i] = -1.0;
    }
    for (i, j) in machine
        .buttons
        .iter()
        .enumerate()
        .cartesian_product(0..n_goals)
        .filter_map(|((j, &button), i)| {
            if (button >> (n_goals - i - 1)) & 1 != 0 {
                (i, j).pipe(Some)
            } else {
                None
            }
        })
    {
        (matrix[i][j], matrix[i + n_goals][j]) = (1.0, -1.0);
    }
    for (i, val) in machine.joltages.iter().copied().map(f64::from).enumerate() {
        (matrix[i][rows - 1], matrix[i + n_goals][rows - 1]) = (val, -val);
    }
    branch_and_bound(matrix).ok_or_eyre("Failed to find solution")
}

fn branch_and_bound(initial: Vec<Vec<f64>>) -> Option<u64> {
    let (mut best_val, mut stack) = (f64::INFINITY, vec![initial]);
    while let Some(current) = stack.pop() {
        match simplex(&current) {
            Some((..0.0, _)) => unreachable!(),
            None => (),
            Some((val, _)) if val >= best_val - EPS => (),
            Some((val, x)) => match x
                .iter()
                .copied()
                .enumerate()
                .find(|&(_, v)| (v - v.round()).abs() > EPS)
            {
                Some((fractional_idx, fractional_val)) => {
                    let width = current[0].len();

                    stack.push({
                        let mut row = vec![0.0; width];
                        (row[fractional_idx], row[width - 1]) = (1.0, fractional_val.floor());
                        let mut a = current.clone();
                        a.push(row);
                        a
                    });
                    stack.push({
                        let mut row = vec![0.0; width];
                        (row[fractional_idx], row[width - 1]) = (-1.0, -fractional_val.ceil());
                        let mut a = current.clone();
                        a.push(row);
                        a
                    });
                }
                None => {
                    best_val = best_val.min(val);
                }
            },
        }
    }
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    match best_val {
        ..0.0 | f64::INFINITY => None,
        val => Some(val.round() as u64),
    }
}

#[allow(clippy::too_many_lines)]
fn simplex(lhs: &[Vec<f64>]) -> Option<(f64, Vec<f64>)> {
    #[inline]
    fn best_key(
        (c_idx, c_key): (usize, (f64, i32)),
        (idx, key): (usize, (f64, i32)),
    ) -> (usize, (f64, i32)) {
        if c_idx == usize::MAX
            || key.0 < c_key.0 - EPS
            || ((key.0 - c_key.0).abs() <= EPS && key.1 < c_key.1)
        {
            (idx, key)
        } else {
            (c_idx, c_key)
        }
    }
    let (width, height) = (lhs[0].len() - 1, lhs.len());
    debug_assert!(width + height < 0xffff);

    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap,
        clippy::cast_possible_truncation
    )]
    let mut n_indices: Vec<i32> = (0..width as i32).collect();
    n_indices.push(-1);

    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap,
        clippy::cast_possible_truncation
    )]
    let mut b_indices: Vec<i32> = (width as i32..(width + height) as i32).collect();

    let mut d = vec![vec![0.0; width + 2]; height + 2];

    for (d_row, lhs_row) in d.iter_mut().zip(lhs.iter()) {
        d_row[..=width].copy_from_slice(lhs_row);
        d_row[width + 1] = -1.0;
    }

    for row in d.iter_mut().take(height) {
        row.swap(width, width + 1);
    }

    d[height][..width].fill(1.0);
    d[height + 1][width] = 1.0;

    let pivot =
        |d: &mut Vec<Vec<f64>>, b_idx: &mut Vec<i32>, n_idx: &mut Vec<i32>, r: usize, s: usize| {
            let k = 1.0 / d[r][s];

            for (i, j) in (0..height + 2)
                .filter(|&i| i != r)
                .cartesian_product(0..width + 2)
                .filter(|&(_, j)| j != s)
            {
                d[i][j] -= d[r][j] * d[i][s] * k;
            }

            for val in &mut d[r] {
                *val *= k;
            }
            for row in &mut d[..] {
                row[s] *= -k;
            }
            d[r][s] = k;

            std::mem::swap(&mut b_idx[r], &mut n_idx[s]);
        };

    let find =
        |d: &mut Vec<Vec<f64>>, b_idx: &mut Vec<i32>, n_idx: &mut Vec<i32>, p_idx: usize| -> bool {
            loop {
                let s = n_idx[0..=width]
                    .iter()
                    .enumerate()
                    .filter(|&(_, &n_elm)| p_idx != 0 || n_elm != -1)
                    .map(|(i, &n_elm)| {
                        let val = d[height + p_idx][i];
                        (i, (val, n_elm))
                    })
                    .reduce(best_key)
                    .expect("filtered out all elements")
                    .0;

                if d[height + p_idx][s] > -EPS {
                    return true;
                }

                let Some((r, _)) = d[0..height]
                    .iter()
                    .enumerate()
                    .filter(|&(_, row)| row[s] > EPS)
                    .map(|(i, row)| {
                        let ratio = row[width + 1] / row[s];
                        (i, (ratio, b_idx[i]))
                    })
                    .reduce(best_key)
                else {
                    return false;
                };

                pivot(d, b_idx, n_idx, r, s);
            }
        };

    let split_r = d
        .iter()
        .enumerate()
        .take(height)
        .skip(1)
        .map(|(idx, row)| (idx, row[width + 1]))
        .min_by(|(_, lhs), (_, rhs)| lhs.total_cmp(rhs))
        .map_or(0, |(idx, _)| idx);

    if d[split_r][width + 1] < -EPS {
        pivot(&mut d, &mut b_indices, &mut n_indices, split_r, width);
        if !find(&mut d, &mut b_indices, &mut n_indices, 1) || d[height + 1][width + 1] < -EPS {
            return None;
        }
        for i in 0..height {
            let best_s = if b_indices[i] == -1 {
                (1..width)
                    .map(|j| (i, (d[i][j], n_indices[j])))
                    .reduce(best_key)
                    .map_or(0, |(i, _)| i)
            } else {
                continue;
            };
            pivot(&mut d, &mut b_indices, &mut n_indices, i, best_s);
        }
    }

    if find(&mut d, &mut b_indices, &mut n_indices, 0) {
        let x = (0..height)
            .filter_map(|i| {
                #[allow(clippy::cast_sign_loss)]
                if b_indices[i] >= 0 && (b_indices[i] as usize) < width {
                    (i, b_indices[i] as usize).pipe(Some)
                } else {
                    None
                }
            })
            .fold(vec![0.0; width], |mut x, (i, j)| {
                x[j] = d[i][width + 1];
                x
            });

        (x.iter().copied().sum(), x).pipe(Some)
    } else {
        None
    }
    .filter(|&(val, _)| val >= 0.0)
}

const EPS: f64 = 1e-9;

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::example("example.txt", 33)]
    #[case::puzzle("input.txt", 16_613)]
    fn finds_solution(#[case] input_path: &str, #[case] expected: u64) -> Result<()> {
        let input: Puzzle = common::read_input!(input_path).parse()?;
        let output = process(input);
        assert_eq!(output, expected);
        Ok(())
    }

    #[rstest]
    #[case("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", 10)]
    #[case("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}", 12)]
    #[case("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}", 11)]
    fn calculates_presses(#[case] machine: Machine, #[case] expected: u64) -> Result<()> {
        assert_eq!(fewest_presses(&machine)?, expected);
        Ok(())
    }
}
