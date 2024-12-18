use std::collections::BinaryHeap;

use common::grid::Grid;
use glam::IVec2;
use itertools::Itertools;
use tap::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
pub struct Puzzle {
    bytes: Vec<IVec2>,
}

impl Puzzle {
    fn map(&self) -> Grid<Option<usize>> {
        let size = IVec2::ONE * if self.bytes.len() < 1_024 { 7 } else { 71 };
        let mut grid = Grid::from_value(None, size);
        for (idx, byte) in self.bytes.iter().copied().enumerate() {
            grid[byte] = Some(idx);
        }
        grid
    }
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        Self {
            bytes: s
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .filter_map(|s| s.split_once(','))
                .map(|(l, r)| {
                    IVec2 {
                        x: l.parse()?,
                        y: r.parse()?,
                    }
                    .pipe(Ok::<_, color_eyre::Report>)
                })
                .try_collect()?,
        }
        .pipe(Ok)
    }
}

pub fn init_tracing() -> color_eyre::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

    let log_filter: EnvFilter = std::env::var(EnvFilter::DEFAULT_ENV)
        .map(String::leak)
        .map(|s| s as &str)
        .or_else(|err| match err {
            std::env::VarError::NotPresent => Ok("day_18=debug"),
            err @ std::env::VarError::NotUnicode(_) => Err(err),
        })?
        .parse()?;
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(log_filter)
        .with_line_number(true)
        .finish()
        .with(tracing_error::ErrorLayer::default());
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

fn reachable(map: &Grid<Option<usize>>, cutoff: usize) -> bool {
    astar(map, cutoff) != usize::MAX
}

fn astar(map: &Grid<Option<usize>>, cutoff: usize) -> usize {
    let goal = map.size() - IVec2::ONE;
    let mut costs = Grid::from_value(usize::MAX, map.size());
    costs[IVec2::ZERO] = 0;
    let mut stack = BinaryHeap::<Entry>::new();
    stack.push(Entry {
        position: IVec2::ZERO,
        cost: 0,
        estimated_cost: 0,
    });
    while let Some(Entry {
        position: pos,
        cost,
        ..
    }) = stack.pop()
    {
        for new_p in [IVec2::X, IVec2::Y, -IVec2::X, -IVec2::Y]
            .into_iter()
            .map(|d| pos + d)
            // .filter(|pos| grid.get(pos).is_some())
            .filter(|pos| match map.get(*pos).copied() {
                Some(Some(idx)) => idx >= cutoff,
                Some(None) => true,
                _ => false,
            })
        {
            let heuristic = {
                let delta = goal - new_p;
                delta.x.abs() + delta.y.abs()
            }
            .try_conv::<usize>()
            .expect("the conversion to succeed");

            let prev = costs[new_p];
            if prev > cost + 1 {
                costs[new_p] = cost + 1;
                stack.push(Entry {
                    position: new_p,
                    cost: cost + 1,
                    estimated_cost: cost + heuristic + 1,
                });
                if new_p == goal {
                    break;
                }
            }
        }
    }

    costs[goal]
}

struct Entry {
    position: IVec2,
    cost: usize,
    estimated_cost: usize,
}

impl std::cmp::PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.estimated_cost.eq(&other.estimated_cost) && self.cost.eq(&other.cost)
    }
}

impl std::cmp::Eq for Entry {}

impl std::cmp::Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        let o = match self.estimated_cost.cmp(&other.estimated_cost) {
            Ordering::Equal => return self.cost.cmp(&other.cost),
            o => o,
        };
        match o {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

impl std::cmp::PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
