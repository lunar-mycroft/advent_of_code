use color_eyre::eyre::{OptionExt, Result};
use glam::{DVec2, I64Vec2};
use itertools::Itertools;
use tap::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
pub struct Puzzle {
    machines: Vec<Machine>,
}

#[derive(Debug, Clone, Copy)]
pub struct Machine {
    a: I64Vec2,
    b: I64Vec2,
    prize: I64Vec2,
}

impl Machine {
    #[allow(clippy::cast_precision_loss)]
    fn moves_to_win_mat(self) -> Option<I64Vec2> {
        #[allow(clippy::cast_possible_truncation)]
        fn to_i64(f: f64) -> Option<i64> {
            let f = (f * 1000.0).round() / 1000.0;
            if f.fract() == 0.0 {
                Some(f as i64)
            } else {
                None
            }
        }
        let m = glam::DMat2::from_cols_array(&[
            self.a.x as f64,
            self.a.y as f64,
            self.b.x as f64,
            self.b.y as f64,
        ]);
        let x: DVec2 = m.inverse()
            * DVec2 {
                x: self.prize.x as f64,
                y: self.prize.y as f64,
            };
        if x.min_element() < 0.0 {
            None
        } else {
            I64Vec2 {
                x: to_i64(x.x)?,
                y: to_i64(x.y)?,
            }
            .pipe(Some)
        }
    }

    #[inline]
    fn moves_to_win_int(self) -> Option<I64Vec2> {
        let b = (self.prize.y * self.a.x - self.prize.x * self.a.y)
            / (self.b.y * self.a.x - self.b.x * self.a.y);
        let a = (self.prize.x - b * self.b.x) / self.a.x;
        if (self.a * a + self.b * b) == self.prize {
            (I64Vec2 { x: a, y: b }).pipe(Some)
        } else {
            None
        }
    }

    fn into_part_2(mut self) -> Self {
        self.prize += 10_000_000_000_000 * I64Vec2::ONE;
        self
    }
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        fn parse_button(s: &str) -> Result<I64Vec2> {
            let (x, y) = s
                .trim()
                .split(|c: char| !c.is_ascii_digit())
                .filter(|s| !s.is_empty())
                .collect_tuple()
                .ok_or_eyre("Failed to parse line")?;
            I64Vec2 {
                x: x.parse()?,
                y: y.parse()?,
            }
            .pipe(Ok)
        }

        Self {
            machines: s
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .tuples()
                .map(|(a, b, p)| {
                    Machine {
                        a: parse_button(a)?,
                        b: parse_button(b)?,
                        prize: parse_button(p)?,
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
            std::env::VarError::NotPresent => Ok("day_13=debug"),
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
