use color_eyre::eyre::OptionExt;
use itertools::Itertools;
use tap::{Pipe, TryConv};

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    ids: Vec<Option<usize>>,
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        let blocks: Vec<_> = s
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| {
                c.to_digit(10)
                    .ok_or_eyre("Non digit found")?
                    .try_conv::<u8>()
                    .map_err(color_eyre::Report::from)
            })
            .try_collect()?;
        Self {
            ids: blocks
                .into_iter()
                .enumerate()
                .fold((true, Vec::new()), |(is_file, mut v), (id, size)| {
                    let value = is_file.then_some(id / 2);
                    v.extend(std::iter::repeat_n(value, size.into()));
                    (!is_file, v)
                })
                .1,
        }
        .pipe(Ok)
    }
}

impl Puzzle {
    fn checksum(&self) -> usize {
        self.ids
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(pos, id)| Some((pos, id?)))
            .map(|(pos, id)| pos * id)
            .sum()
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for id in &self.ids {
            match *id {
                Some(id) => write!(f, "{id}"),
                None => write!(f, "."),
            }?;
        }
        Ok(())
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
