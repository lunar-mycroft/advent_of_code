use std::borrow::Cow;

use color_eyre::eyre::{bail, OptionExt};
use fxhash::FxHashMap;
use itertools::Itertools;
use tap::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
pub struct Puzzle {
    state: FxHashMap<Wire, bool>,
    operations: FxHashMap<Wire, Gate>,
    other_wires: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Wire {
    X(u8),
    Y(u8),
    Z(u8),
    Other(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Operation {
    Xor,
    Or,
    And,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Gate {
    left: Wire,
    op: Operation,
    right: Wire,
}

impl Gate {
    fn reveresed(mut self) -> Self {
        std::mem::swap(&mut self.left, &mut self.right);
        self
    }
}

impl Puzzle {
    fn output_of(&self, gate: Gate) -> Option<Wire> {
        self.operations
            .iter()
            .find_map(|(w, other)| (*other == gate || *other == gate.reveresed()).then_some(w))
            .copied()
    }

    fn wire_str(&self, wire: Wire) -> Cow<'_, str> {
        match wire {
            Wire::X(n) => format!("x{n:0>2}").into(),
            Wire::Y(n) => format!("y{n:0>2}").into(),
            Wire::Z(n) => format!("z{n:0>2}").into(),
            Wire::Other(n) => (&self.other_wires[n as usize]).into(),
        }
    }

    fn eval(&self, wire: Wire) -> bool {
        match (
            self.state.get(&wire).copied(),
            self.operations.get(&wire).copied(),
        ) {
            (Some(b), _) => b,
            (
                None,
                Some(Gate {
                    left,
                    op: Operation::Xor,
                    right,
                }),
            ) => self.eval(left) != self.eval(right),
            (
                None,
                Some(Gate {
                    left,
                    op: Operation::And,
                    right,
                }),
            ) => self.eval(left) && self.eval(right),
            (
                None,
                Some(Gate {
                    left,
                    op: Operation::Or,
                    right,
                }),
            ) => self.eval(left) || self.eval(right),
            (None, None) => unreachable!(),
        }
    }
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        let (gates, operations) = s
            .split_once("\n\n")
            .ok_or_eyre("Couldn't seperate chunks")?;
        let mut other_map = FxHashMap::default();
        let mut parse_wire = |w: &str| {
            let current_len = other_map.len();
            match w {
                s if s.starts_with('x') => s[1..].parse().map(Wire::X)?,
                s if s.starts_with('y') => s[1..].parse().map(Wire::Y)?,
                s if s.starts_with('z') => s[1..].parse().map(Wire::Z)?,
                s => other_map
                    .entry(s.to_owned())
                    .or_insert(current_len)
                    .pipe(|n| *n)
                    .try_conv::<u8>()
                    .map(Wire::Other)?,
            }
            .pipe(Ok::<_, color_eyre::Report>)
        };
        Self {
            state: gates
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|line| {
                    let (l, r) = line
                        .split_once(": ")
                        .ok_or_eyre("Couldn't split gate line")?;
                    (parse_wire(l)?, r == "1").pipe(Ok::<_, color_eyre::Report>)
                })
                .try_collect()?,
            operations: operations
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|line| {
                    let (l, op, r, res) = line
                        .split_whitespace()
                        .filter(|s| *s != "->")
                        .map(str::to_owned)
                        .collect_tuple::<(_, _, _, _)>()
                        .ok_or_eyre("Incorrect number of elements in line")?;
                    (
                        parse_wire(&res)?,
                        Gate {
                            left: parse_wire(&l)?,
                            op: op.parse()?,
                            right: parse_wire(&r)?,
                        },
                    )
                        .pipe(Ok::<_, color_eyre::Report>)
                })
                .try_collect()?,
            other_wires: other_map
                .into_iter()
                .sorted_unstable_by_key(|(_, n)| *n)
                .map(|(s, _)| s)
                .collect(),
        }
        .pipe(Ok)
    }
}

impl std::str::FromStr for Operation {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "XOR" => Ok(Self::Xor),
            "OR" => Ok(Self::Or),
            "AND" => Ok(Self::And),
            s => bail!("{s:?} is not a valid gate"),
        }
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
