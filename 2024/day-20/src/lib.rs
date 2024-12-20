use std::collections::BinaryHeap;

use color_eyre::eyre::OptionExt;
use glam::IVec2;
use tap::prelude::*;

use common::grid::Grid;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    map: Grid<u8>,
    start: IVec2,
    end: IVec2,
}

impl Puzzle {
    #[must_use]
    pub fn astar(&self) -> Costs {
        let mut costs = Costs::new(self.map.size());
        assert!(costs.replace_if_lt(self.start, 0));
        let mut stack = BinaryHeap::<Entry>::new();
        stack.push(Entry {
            position: self.start,
            cost: 0,
            estimated_cost: 0,
        });
        while let Some(Entry {
            position: pos,
            cost,
            ..
        }) = stack.pop()
        {
            for new_p in [
                pos + IVec2::X,
                pos + IVec2::Y,
                pos - IVec2::X,
                pos - IVec2::Y,
            ]
            .into_iter()
            .filter(|p| self.map[*p] != b'#')
            {
                let heuristic = {
                    let delta = self.end - new_p;
                    delta.x.abs() + delta.y.abs()
                }
                .try_conv::<u32>()
                .expect("the conversion to succeed");

                if costs.replace_if_lt(new_p, cost + 1) {
                    stack.push(Entry {
                        position: new_p,
                        cost: cost + 1,
                        estimated_cost: cost + heuristic + 1,
                    });
                }
            }
        }
        costs
    }
}

pub struct Costs(Grid<u32>);

impl Costs {
    fn new(size: IVec2) -> Self {
        Grid::from_value(u32::MAX, size).pipe(Self)
    }
    fn replace_if_lt(&mut self, pos: IVec2, cost: u32) -> bool {
        let Some(tile) = self.0.get_mut(pos) else {
            return false;
        };
        if *tile > cost {
            *tile = cost;
            true
        } else {
            false
        }
    }

    fn get(&self, pos: IVec2) -> Option<u32> {
        self.0.get(pos).filter(|&v| *v < u32::MAX).copied()
    }
}

pub struct Entry {
    position: IVec2,
    cost: u32,
    estimated_cost: u32,
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

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        let map: Grid<u8> = s.parse()?;
        Self {
            start: map
                .iter()
                .copied()
                .zip(map.positions())
                .find_map(|(c, p)| (c == b'S').then_some(p))
                .ok_or_eyre("No start found")?,
            end: map
                .iter()
                .copied()
                .zip(map.positions())
                .find_map(|(c, p)| (c == b'E').then_some(p))
                .ok_or_eyre("No start found")?,
            map,
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
            std::env::VarError::NotPresent => Ok("day_20=debug"),
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
