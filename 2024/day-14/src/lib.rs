use std::cmp::Ordering;

use color_eyre::eyre::OptionExt;
use glam::IVec2;
use itertools::Itertools;
use tap::{Pipe, TryConv};

pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
pub struct Puzzle {
    robots: Vec<Robot>,
}

impl Puzzle {
    /// give the _approximate_ mean of the robots positions
    fn mean(&self) -> IVec2 {
        self.robots.iter().map(|r| r.position).sum::<IVec2>()
            / self
                .robots
                .len()
                .try_conv::<i32>()
                .expect("number of robots to be less than i32::MAX")
    }

    /// gives the __approximate_ mean of the robots positions
    fn variance(&self) -> IVec2 {
        let mean = self.mean();
        self.robots
            .iter()
            .map(|r| IVec2 {
                x: (r.position.x - mean.x).pow(2),
                y: (r.position.y - mean.y).pow(2),
            })
            .sum()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    position: IVec2,
    velocity: IVec2,
}

impl Robot {
    fn tick(&mut self, size: IVec2) {
        self.position += self.velocity;
        if self.position.x < 0 {
            self.position.x += size.x;
        } else if self.position.x >= size.x {
            self.position.x -= size.x;
        }
        if self.position.y < 0 {
            self.position.y += size.y;
        } else if self.position.y >= size.y {
            self.position.y -= size.y;
        }
    }

    fn quadrent(&self, size: IVec2) -> (Ordering, Ordering) {
        let mid = size / 2;
        (self.position.x.cmp(&mid.x), self.position.y.cmp(&mid.y))
    }
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        Self {
            robots: s
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|line| {
                    let (p, v) = line.split_once(' ')?;
                    (
                        p.strip_prefix("p=")?.split_once(',')?,
                        v.strip_prefix("v=")?.split_once(',')?,
                    )
                        .pipe(Some)
                })
                .map(|opt| {
                    let ((px, py), (vx, vy)) = opt.ok_or_eyre("Incorrect line")?;
                    Robot {
                        position: IVec2 {
                            x: px.parse()?,
                            y: py.parse()?,
                        },
                        velocity: IVec2 {
                            x: vx.parse()?,
                            y: vy.parse()?,
                        },
                    }
                    .pipe(Ok::<_, color_eyre::Report>)
                })
                .try_collect()?,
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
            std::env::VarError::NotPresent => Ok("day_14=debug"),
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
