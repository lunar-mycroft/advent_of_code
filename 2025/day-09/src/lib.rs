use color_eyre::eyre::{ensure, OptionExt};
use glam::IVec2;
use itertools::Itertools;
use tap::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    tiles: Vec<IVec2>,
}

fn area(a: IVec2, b: IVec2) -> u64 {
    let v = (a - b).abs() + IVec2::ONE;
    u64::try_from(v.x).expect("Known non-negative")
        * u64::try_from(v.y).expect("Known non-negative")
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        let tiles: Vec<_> = s
            .lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| {
                let (x, y) = s.split_once(',').ok_or_eyre("Missing ','")?;
                IVec2 {
                    x: x.parse()?,
                    y: y.parse()?,
                }
                .pipe(Ok::<_, color_eyre::Report>)
            })
            .try_collect()?;
        ensure!(tiles.len() > 1, "input too short");
        ensure!(
            tiles.iter().all(|tile| tile.x >= 0 && tile.y >= 0),
            "Negative tile found"
        );
        Ok(Self { tiles })
    }
}

pub fn init_tracing() -> color_eyre::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

    let log_filter: EnvFilter = std::env::var(EnvFilter::DEFAULT_ENV)
        .map(String::leak)
        .map(|s| s as &str)
        .or_else(|err| match err {
            std::env::VarError::NotPresent => Ok("day_09=debug"),
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
