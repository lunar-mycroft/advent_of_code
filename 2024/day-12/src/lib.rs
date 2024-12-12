use std::collections::HashSet;

use glam::IVec2;
use tap::prelude::*;

use common::grid::Grid;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    plots: Grid<u8>,
}

#[derive(Debug)]
pub struct Region(HashSet<IVec2>);

impl Puzzle {
    fn region(&self, start: IVec2) -> Option<Region> {
        let (mut visited, mut fringe) = (HashSet::new(), HashSet::new());
        let plant = self.plots.get(start).copied()?;
        fringe.insert(start);
        while !fringe.is_empty() {
            visited.extend(fringe.iter().copied());
            fringe = fringe
                .into_iter()
                .flat_map(|pos| {
                    [
                        pos + IVec2::X,
                        pos + IVec2::Y,
                        pos - IVec2::X,
                        pos - IVec2::Y,
                    ]
                })
                .filter(|pos| !visited.contains(pos))
                .filter(|pos| self.plots.get(*pos).copied() == Some(plant))
                .collect();
        }
        visited
            .union(&fringe)
            .copied()
            .collect::<HashSet<_>>()
            .pipe(Region)
            .pipe(Some)
    }

    fn regions(&self) -> impl Iterator<Item = Region> + '_ {
        let mut seen = HashSet::new();
        let mut positions = self.plots.positions();
        std::iter::from_fn(move || loop {
            let pos = positions.next()?;
            if seen.contains(&pos) {
                continue;
            }
            let region = self.region(pos)?;
            seen.extend(region.0.iter().copied());
            break Some(region);
        })
    }
}

impl Region {
    fn area(&self) -> usize {
        self.0.len()
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
