use color_eyre::eyre::{ensure, OptionExt};
use glam::IVec2;
use tap::prelude::*;

use common::grid::Grid;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    grid: Grid<u8>,
    start: i32,
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        let grid: Grid<u8> = s.parse()?;
        ensure!(grid.iter().all(|b| matches!(b, b'^' | b'.' | b'S')));
        Self {
            start: (0..grid.size().x)
                .find(|&x| grid[IVec2::new(x, 0)] == b'S')
                .ok_or_eyre("Missing start")?,
            grid,
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
            std::env::VarError::NotPresent => Ok("day_07=debug"),
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
