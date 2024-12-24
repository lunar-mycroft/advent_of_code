use color_eyre::eyre::OptionExt;
use fxhash::FxHashMap;
use itertools::Itertools;
use tap::Pipe;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    state: FxHashMap<String, bool>,
    operations: FxHashMap<String, (String, String, String)>,
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
