use color_eyre::eyre::{OptionExt, Result};
use fxhash::FxHashMap as HashMap;
use itertools::Itertools;
use tap::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    connections: Vec<Vec<u16>>,
    you: Option<u16>,
    svr: Option<u16>,
    dac: Option<u16>,
    fft: Option<u16>,
    out: u16,
}

impl Puzzle {
    fn topological_order(&self) -> Vec<u16> {
        let (mut incoming_edges, mut order, mut total_edges) =
            (vec![0u32; self.connections.len()], Vec::new(), 0);
        for &snk in self.connections.iter().flat_map(|v| v.iter()) {
            incoming_edges[snk as usize] += 1;
            total_edges += 1;
        }
        let mut queue = (0..self.connections.len())
            .filter(|&idx| incoming_edges[idx] == 0)
            .collect_vec();
        while let Some(node) = queue.pop() {
            for &neighbor in &self.connections[node] {
                incoming_edges[neighbor as usize] -= 1;
                total_edges -= 1;
                if incoming_edges[neighbor as usize] == 0 {
                    queue.push(neighbor as usize);
                }
            }
            order.push(
                node.try_conv()
                    .expect("there aren't enough nodes to overflow"),
            );
        }
        debug_assert_eq!(total_edges, 0);
        order
    }

    fn num_paths(&self, from: u16, to: u16, order: &[u16]) -> u64 {
        let mut res = vec![0u64; self.connections.len()];
        res[from as usize] = 1;
        for &node in order {
            for &neighbor in &self.connections[node as usize] {
                res[neighbor as usize] += res[node as usize];
            }
        }
        res[to as usize]
    }
}
impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        let PuzzleBuilder { strs, connections } = PuzzleBuilder::from_str(s)?;
        Self {
            you: strs.get("you").copied().map(u16::try_from).transpose()?,
            svr: strs.get("svr").copied().map(u16::try_from).transpose()?,
            out: (*strs.get("out").ok_or_eyre("Missing out")?).try_conv()?,
            dac: strs.get("dac").copied().map(u16::try_from).transpose()?,
            fft: strs.get("fft").copied().map(u16::try_from).transpose()?,
            connections,
        }
        .pipe(Ok)
    }
}

#[derive(Default)]
struct PuzzleBuilder<'s> {
    strs: HashMap<&'s str, u16>,
    connections: Vec<Vec<u16>>,
}

impl<'s> PuzzleBuilder<'s> {
    fn from_str(s: &'s str) -> color_eyre::Result<Self> {
        let mut builder = Self {
            strs: HashMap::default(),
            connections: Vec::new(),
        };
        for line in s.lines().map(str::trim).filter(|s| !s.is_empty()) {
            let (src, snk) = line
                .split_once(": ")
                .ok_or_eyre("Missing source and sinks")?;
            let src = builder.index_of(src)?;
            for seg in snk.split_whitespace() {
                let neighbor = builder.index_of(seg)?;
                builder.connections[src as usize].push(neighbor);
            }
        }
        Ok(builder)
    }

    fn index_of(&mut self, machine: &'s str) -> Result<u16> {
        let n: u16 = self.strs.len().try_conv()?;
        match self.strs.entry(machine) {
            std::collections::hash_map::Entry::Occupied(entry) => Ok(*entry.get()),
            std::collections::hash_map::Entry::Vacant(entry) => {
                self.connections.push(vec![]);
                entry.insert(n);
                Ok(n)
            }
        }
    }
}

pub fn init_tracing() -> color_eyre::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

    let log_filter: EnvFilter = std::env::var(EnvFilter::DEFAULT_ENV)
        .map(String::leak)
        .map(|s| s as &str)
        .or_else(|err| match err {
            std::env::VarError::NotPresent => Ok("day_11=debug"),
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
