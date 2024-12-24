use std::ops::Deref;

use color_eyre::eyre::OptionExt;
use fxhash::FxHashMap;
use itertools::Itertools;
use tap::Pipe;

pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
pub struct Puzzle {
    state: FxHashMap<String, bool>,
    operations: FxHashMap<String, (String, String, String)>,
}

impl Puzzle {
    fn as_add(mut self, mut x: u64, mut y: u64) -> Self {
        for idx in 0..45 {
            let (k_x, k_y) = (format!("x{idx:0>2}"), format!("y{idx:0>2}"));
            self.state.insert(k_x, x & 1 == 1);
            self.state.insert(k_y, y & 1 == 1);
            x >>= 1;
            y >>= 1;
            if x == 0 && y == 0 {
                break;
            }
        }
        self
    }

    fn output_of(&self, lhs: &str, op: &str, rhs: &str) -> Option<&str> {
        self.operations
            .iter()
            .find_map(|(w, (l, o, r))| {
                ((l == lhs && o == op && r == rhs) || (r == lhs && o == op && l == rhs))
                    .then_some(w)
            })
            .map(String::deref)
    }

    fn inspect(&self, wire: &str, depth: u8) -> Option<String> {
        let Some((lhs, op, rhs)) = self.operations.get(wire) else {
            return Some(wire.to_owned());
        };
        if depth == 0 {
            format!("{lhs} {op} {rhs}").pipe(Some)
        } else {
            format!(
                "({}) {op} ({})",
                self.inspect(lhs, depth - 1)?,
                self.inspect(rhs, depth - 1)?
            )
            .pipe(Some)
        }
    }

    fn compute(mut self) -> u64 {
        while self.operations.keys().any(|id| id.starts_with('z')) {
            let mut new_ops = FxHashMap::default();
            for (dest, (lhs, op, rhs)) in self.operations {
                let (Some(lhs), Some(rhs)) =
                    (self.state.get(&lhs).copied(), self.state.get(&rhs).copied())
                else {
                    new_ops.insert(dest, (lhs, op, rhs));
                    continue;
                };
                let out = match op.as_str() {
                    "OR" => lhs || rhs,
                    "XOR" => lhs != rhs,
                    "AND" => lhs && rhs,
                    _ => unreachable!(),
                };
                self.state.insert(dest, out);
            }
            self.operations = new_ops;
        }
        self.z()
    }

    fn eval(&self, wire: &str) -> bool {
        match (
            self.state.get(wire),
            self.operations
                .get(wire)
                .map(|(l, o, r)| (l, o.as_str(), r)),
        ) {
            (Some(b), _) => *b,
            (None, Some((lhs, "XOR", rhs))) => self.eval(lhs) != self.eval(rhs),
            (None, Some((lhs, "AND", rhs))) => self.eval(lhs) && self.eval(rhs),
            (None, Some((lhs, "OR", rhs))) => self.eval(lhs) || self.eval(rhs),
            (None, other) => {
                dbg!(other, wire);
                unreachable!()
            }
        }
    }

    fn var(&self, name: char) -> u64 {
        self.state
            .iter()
            .filter(|(s, _)| s.starts_with(name))
            .sorted_unstable()
            .rev()
            .fold(0, |mut res, (_, bit)| {
                res <<= 1;
                res |= u64::from(*bit);
                res
            })
    }

    fn x(&self) -> u64 {
        self.var('x')
    }

    fn y(&self) -> u64 {
        self.var('y')
    }

    fn z(&self) -> u64 {
        self.var('z')
    }
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        let (gates, operations) = s
            .split_once("\n\n")
            .ok_or_eyre("Couldn't seperate chunks")?;
        Self {
            state: gates
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|line| {
                    let (l, r) = line
                        .split_once(": ")
                        .ok_or_eyre("Couldn't split gate line")?;
                    (l.to_owned(), r == "1").pipe(Ok::<_, color_eyre::Report>)
                })
                .try_collect()?,
            operations: operations
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|line| {
                    line.split_whitespace()
                        .filter(|s| *s != "->")
                        .map(str::to_owned)
                        .collect_tuple::<(_, _, _, _)>()
                        .ok_or_eyre("Incorrect number of elements in line")
                })
                .map_ok(|(a, b, c, d)| (d, (a, b, c)))
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
            std::env::VarError::NotPresent => Ok("day_24=debug"),
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
