use std::collections::HashSet;

use glam::IVec2;
use tap::prelude::*;

pub mod part1;
pub mod part2;

pub fn init_tracing() -> color_eyre::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

    let log_filter: EnvFilter = std::env::var(EnvFilter::DEFAULT_ENV)
        .map(String::leak)
        .map(|s| s as &str)
        .or_else(|err| match err {
            std::env::VarError::NotPresent => Ok("day_06=debug"),
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

fn patrol(
    mut pos: IVec2,
    mut dir: IVec2,
    blocks: &HashSet<IVec2>,
) -> impl Iterator<Item = (IVec2, IVec2)> + '_ {
    std::iter::from_fn(move || {
        let old_dir = dir;
        while blocks.contains(&(pos + dir)) {
            dir = turn_right(dir);
        }
        let old_pos = pos;
        pos += dir;
        Some((old_pos, old_dir))
    })
}

fn in_map(pos: IVec2, size: IVec2) -> bool {
    (0..size.x).contains(&pos.x) && (0..size.y).contains(&pos.y)
}

fn guard(input: &str) -> Option<(IVec2, IVec2)> {
    chars_and_coords(input).find_map(|(pos, c)| match c {
        '^' => (pos, IVec2 { x: 0, y: -1 }).pipe(Some),
        'v' => (pos, IVec2 { x: 0, y: 1 }).pipe(Some),
        '>' => (pos, IVec2 { x: 1, y: 0 }).pipe(Some),
        '<' => (pos, IVec2 { x: -1, y: 0 }).pipe(Some),
        _ => None,
    })
}

fn turn_right(dir: IVec2) -> IVec2 {
    match dir {
        IVec2 { x: 0, y: 1 } => IVec2 { x: -1, y: 0 },
        IVec2 { x: 1, y: 0 } => IVec2 { x: 0, y: 1 },
        IVec2 { x: 0, y: -1 } => IVec2 { x: 1, y: 0 },
        IVec2 { x: -1, y: 0 } => IVec2 { x: 0, y: -1 },
        v => panic!("Unsupported direction {v}"),
    }
}

fn chars_and_coords(input: &str) -> impl Iterator<Item = (IVec2, char)> + '_ {
    input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    IVec2 {
                        x: x.try_conv().expect("input to be smaller than 2^32"),
                        y: y.try_conv::<i32>().expect("input to be smaller than 2^32"),
                    },
                    c,
                )
            })
        })
}

fn blocks(input: &str) -> impl Iterator<Item = IVec2> + '_ {
    chars_and_coords(input).filter_map(|(pos, c)| if c == '#' { Some(pos) } else { None })
}
