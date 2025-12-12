use color_eyre::eyre::{ensure, eyre, OptionExt};
use common::grid::Grid;
use glam::IVec2;
use itertools::Itertools;
use tap::prelude::*;

pub mod process;

#[derive(Debug)]
pub struct Puzzle {
    presents: [Grid<u8>; 6],
    regions: Vec<Region>,
}

#[derive(Debug, Clone, Copy)]
pub struct Region {
    size: IVec2,
    quantities: [u8; 6],
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        let [presents @ .., regions] = s
            .split("\n\n")
            .collect_array::<7>()
            .ok_or_eyre("Incorrect number of sections")?;
        Self {
            presents: presents
                .into_iter()
                .map(str::trim)
                .map(|p| {
                    let (_, p) = p.split_once(":\n").ok_or_eyre("Missing :")?;
                    Grid::from_str(p).map_err(color_eyre::Report::from)
                })
                .try_collect::<_, Vec<_>, _>()?
                .try_conv()
                .expect("Known correct length"),
            regions: regions.trim().lines().map(Region::from_str).try_collect()?,
        }
        .pipe(Ok)
    }
}

impl std::str::FromStr for Region {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size, quantities) = s.trim().split_once(": ").ok_or_eyre("Missing :")?;
        let (x, y) = size.split_once('x').ok_or_eyre("Missinx x")?;
        let size = IVec2 {
            x: x.parse()?,
            y: y.parse()?,
        };
        ensure!(
            matches!(
                size,
                IVec2 {
                    x: 0..100,
                    y: 0..100
                }
            ),
            "Invalid size {size}"
        );
        Self {
            size,
            quantities: quantities
                .split_whitespace()
                .map(u8::from_str)
                .try_collect::<_, Vec<_>, _>()?
                .try_conv()
                .map_err(|v: Vec<u8>| {
                    eyre!("Wrong number of quantities! Expected 6, got {}", v.len())
                })?,
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
