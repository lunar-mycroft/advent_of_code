use color_eyre::eyre::OptionExt;
use common::grid::Grid;
use glam::IVec2;
use tap::Pipe;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    map: Grid<u8>,
    start: IVec2,
    end: IVec2,
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        let map: Grid<u8> = s.parse()?;
        Self {
            start: map
                .positions()
                .zip(map.iter().copied())
                .find_map(|(p, c)| (c == b'S').then_some(p))
                .ok_or_eyre("Missing start")?,
            end: map
                .positions()
                .zip(map.iter().copied())
                .find_map(|(p, c)| (c == b'E').then_some(p))
                .ok_or_eyre("Missing end")?,
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
            std::env::VarError::NotPresent => Ok("day_16=debug"),
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
