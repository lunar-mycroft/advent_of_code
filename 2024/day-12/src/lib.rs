use std::collections::HashSet;

use glam::IVec2;
use itertools::Itertools;
use tap::prelude::*;

use common::grid::Grid;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    plots: Grid<u8>,
}

#[derive(Debug)]
pub struct Region {
    perimiter: usize,
    area: usize,
    corners: usize,
}

impl Puzzle {
    fn region(&self, start: IVec2) -> Option<(Region, HashSet<IVec2>)> {
        let plant = self.plots.get(start).copied()?;
        let (mut region, mut plots, mut plot_stack) = (
            Region {
                perimiter: 4,
                area: 1,
                corners: self.corners(start, plant),
            },
            HashSet::new(),
            Vec::new(),
        );
        plot_stack.push(start);
        plots.insert(start);
        while let Some(pos) = plot_stack.pop() {
            for neighbor in [
                pos + IVec2::X,
                pos + IVec2::Y,
                pos - IVec2::X,
                pos - IVec2::Y,
            ]
            .into_iter()
            .filter(|pos| self.plots.get(*pos).copied() == Some(plant))
            {
                region.perimiter -= 1;
                if plots.contains(&neighbor) {
                    continue;
                }
                region.area += 1;
                region.perimiter += 4;
                region.corners += self.corners(neighbor, plant);
                plot_stack.push(neighbor);
                plots.insert(neighbor);
            }
        }
        (region, plots).pipe(Some)
    }

    fn regions(&self) -> impl Iterator<Item = Region> + '_ {
        let mut seen = HashSet::new();
        let mut positions = self.plots.positions();
        std::iter::from_fn(move || loop {
            let pos = positions.next()?;
            if seen.contains(&pos) {
                continue;
            }
            let (region, plots) = self.region(pos)?;
            seen.extend(plots.iter().copied());
            break Some(region);
        })
    }

    fn corners(&self, pos: IVec2, plant: u8) -> usize {
        [IVec2::NEG_Y, IVec2::X, IVec2::Y, IVec2::NEG_X]
            .into_iter()
            .circular_tuple_windows()
            .map(|(a, b)| {
                (
                    self.plots.get(pos + a).copied(),
                    self.plots.get(pos + b).copied(),
                    self.plots.get(pos + a + b).copied(),
                )
            })
            .filter(|(left, right, mid)| {
                (*left != Some(plant) && *right != Some(plant))
                    || (*left == Some(plant) && *right == Some(plant) && *mid != Some(plant))
            })
            .count()
    }
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        s.parse().map(|plots| Self { plots }).map_err(From::from)
    }
}

pub fn init_tracing() -> color_eyre::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

    let log_filter: EnvFilter = std::env::var(EnvFilter::DEFAULT_ENV)
        .map(String::leak)
        .map(|s| s as &str)
        .or_else(|err| match err {
            std::env::VarError::NotPresent => Ok("day_12=debug"),
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
