use color_eyre::eyre::{ensure, OptionExt};
use glam::I64Vec3 as IVec3;
use itertools::Itertools;
use rayon::slice::ParallelSliceMut;
use tap::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    boxes: Vec<IVec3>,
}

impl Puzzle {
    #[must_use]
    pub fn by_distance(&self) -> Vec<(usize, usize)> {
        let mut res = self
            .boxes
            .iter()
            .enumerate()
            .tuple_combinations::<(_, _)>()
            .collect_vec();
        res.par_sort_unstable_by_key(|&((_, u), (_, v))| u.distance_squared(*v));
        res.into_iter().map(|((i, _), (j, _))| (i, j)).collect()
    }

    #[must_use]
    pub fn n_by_distance(&self, n: usize) -> Vec<(usize, usize)> {
        let mut res = self
            .boxes
            .iter()
            .enumerate()
            .tuple_combinations::<(_, _)>()
            .collect_vec();
        let v = res
            .select_nth_unstable_by_key(n, |&((_, u), (_, v))| u.distance_squared(*v))
            .0
            .iter()
            .map(|&((i, _), (j, _))| (i, j))
            .collect_vec();
        debug_assert_eq!(v.len(), n);
        v
    }
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

#[derive(Debug)]
struct Dsu {
    nodes: Box<[Circuit]>,
}

#[derive(Debug)]
struct Circuit {
    parent: usize,
    size: usize,
}

impl Dsu {
    fn new(len: usize) -> Self {
        Self {
            nodes: (0..len).map(|parent| Circuit { parent, size: 1 }).collect(),
        }
    }
    fn parent(&mut self, mut x: usize) -> Option<usize> {
        loop {
            let parent = self.nodes.get(x)?.parent;
            if parent == x {
                break Some(parent);
            }
            (x, self.nodes[x].parent) = (parent, self.nodes[parent].parent);
        }
    }

    fn unite(&mut self, u: usize, v: usize) -> Option<usize> {
        let (larger, smaller) = {
            let (pu, pv) = (self.parent(u)?, self.parent(v)?);
            if self.nodes[pu].size < self.nodes[pv].size {
                (pv, pu)
            } else {
                (pu, pv)
            }
        };
        if larger == smaller {
            return Some(self.nodes[u].size);
        }

        self.nodes[smaller].parent = larger;
        self.nodes[larger].size += self.nodes[smaller].size;
        Some(self.nodes[larger].size)
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
