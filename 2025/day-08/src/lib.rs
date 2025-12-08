use std::collections::HashMap;

use color_eyre::eyre::{ensure, OptionExt};
use glam::I64Vec3 as IVec3;
use itertools::Itertools;
use tap::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    boxes: Vec<IVec3>,
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        let boxes: Vec<_> = s
            .lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.split(',')
                    .collect_tuple::<(_, _, _)>()
                    .ok_or_eyre("Wrong number of coordinates")
            })
            .map(|res| {
                res.and_then(|(x, y, z)| {
                    IVec3 {
                        x: x.parse()?,
                        y: y.parse()?,
                        z: z.parse()?,
                    }
                    .pipe(Ok)
                })
            })
            .try_collect()?;
        ensure!(boxes.iter().copied().all_unique());
        Self { boxes }.pipe(Ok)
    }
}

#[derive(Debug, Default)]
struct Dsu {
    parents: HashMap<IVec3, IVec3>,
}

impl Dsu {
    fn parent(&self, mut u: IVec3) -> IVec3 {
        while let Some(v) = self.parents.get(&u) {
            u = *v;
        }
        u
    }

    fn unite(&mut self, u: IVec3, v: IVec3) -> usize {
        let (pu, pv) = (self.parent(u), self.parent(v));
        if pu == pv {
            0
        } else {
            self.parents.insert(pu, pv);
            1
        }
    }
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
