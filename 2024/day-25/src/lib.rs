use common::grid::Grid;
use glam::IVec2;
use itertools::Itertools;

pub mod part1;

#[derive(Debug)]
pub struct Puzzle {
    items: Vec<Grid<char>>,
}

fn is_key(item: &Grid<char>) -> bool {
    (0..item.size().x)
        .map(|x| item.size().with_x(x) - IVec2::Y)
        .all(|pos| *item.get(pos).expect("known good coord") == '#')
}

#[inline]
fn is_lock(item: &Grid<char>) -> bool {
    !is_key(item)
}

#[allow(clippy::cast_sign_loss)]
fn heights(item: &Grid<char>) -> [u8; 5] {
    let mut res = [0; 5];
    for (pos, &c) in item.positions().zip(item.iter()) {
        match c {
            '#' => res[pos.x as usize] += 1,
            '.' => (),
            _ => unreachable!(),
        }
    }
    for n in &mut res {
        *n -= 1;
    }
    res
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        s.split("\n\n")
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::parse)
            .try_collect()
            .map(|items| Self { items })
            .map_err(From::from)
    }
}

pub fn init_tracing() -> color_eyre::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

    let log_filter: EnvFilter = std::env::var(EnvFilter::DEFAULT_ENV)
        .map(String::leak)
        .map(|s| s as &str)
        .or_else(|err| match err {
            std::env::VarError::NotPresent => Ok("day_25=debug"),
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
