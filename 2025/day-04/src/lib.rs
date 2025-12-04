use glam::IVec2;
use tap::prelude::*;

pub mod part1;
pub mod part2;

const NEIGBORS: [IVec2; 8] = [
    IVec2::new(-1, -1),
    IVec2::new(-1, 0),
    IVec2::new(-1, 1),
    IVec2::new(0, -1),
    IVec2::new(0, 1),
    IVec2::new(1, -1),
    IVec2::new(1, 0),
    IVec2::new(1, 1),
];

#[derive(Debug)]
pub struct Puzzle {
    pub grid: common::grid::Grid<u8>,
}

impl Puzzle {
    fn reachable(&self, center: IVec2) -> bool {
        match self.grid.get(center).copied() {
            Some(b'@') => {
                NEIGBORS
                    .iter()
                    .copied()
                    .map(|pos| pos + center)
                    .filter(|&pos| self.grid.get(pos).is_some_and(|&b| b == b'@'))
                    .count()
                    < 4
            }
            Some(_) | None => false,
        }
    }
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        Self { grid: s.parse()? }.pipe(Ok)
    }
}

pub fn init_tracing() -> color_eyre::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

    let log_filter: EnvFilter = std::env::var(EnvFilter::DEFAULT_ENV)
        .map(String::leak)
        .map(|s| s as &str)
        .or_else(|err| match err {
            std::env::VarError::NotPresent => Ok("day_04=debug"),
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
