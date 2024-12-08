use std::collections::HashMap;

use color_eyre::eyre::OptionExt;
use glam::IVec2;
use itertools::Itertools;
use tap::prelude::*;

pub mod part1;
pub mod part2;

fn parse(input: &str) -> color_eyre::Result<(IVec2, HashMap<char, Vec<IVec2>>)> {
    (
        IVec2 {
            x: input
                .lines()
                .map(str::len)
                .max()
                .ok_or_eyre("No lines found")?
                .try_into()?,
            y: input.lines().count().try_into()?,
        },
        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    (
                        IVec2 {
                            x: x.try_into().expect(""),
                            y: y.try_into().expect(""),
                        },
                        c,
                    )
                        .pipe(Ok::<_, color_eyre::Report>)
                })
            })
            .filter_ok(|(_, c)| *c != '.')
            .try_fold(HashMap::<char, Vec<_>>::new(), |mut acc, res| {
                let (pos, c) = res?;
                acc.entry(c).or_default().push(pos);
                Ok::<_, color_eyre::Report>(acc)
            })?,
    )
        .pipe(Ok)
}

pub fn init_tracing() -> color_eyre::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

    let log_filter: EnvFilter = std::env::var(EnvFilter::DEFAULT_ENV)
        .map(String::leak)
        .map(|s| s as &str)
        .or_else(|err| match err {
            std::env::VarError::NotPresent => Ok("day_08=debug"),
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
