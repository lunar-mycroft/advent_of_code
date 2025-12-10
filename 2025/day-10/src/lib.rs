use color_eyre::eyre::{bail, ensure, Context, OptionExt};
use itertools::Itertools;
use tap::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    machines: Vec<Machine>,
}

#[derive(Debug)]
pub struct Machine {
    lights: u16,
    buttons: Vec<u16>,
    joltages: Vec<u16>,
}

impl std::str::FromStr for Machine {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix('[')
            .ok_or_eyre("Missing prefix")?
            .strip_suffix('}')
            .ok_or_eyre("Missing suffix")?;
        let (goal, rest) = s
            .split_once("] (")
            .ok_or_eyre("Missing goal-rest seperator")?;
        ensure!(
            goal.as_bytes()
                .iter()
                .copied()
                .all(|b| matches!(b, b'.' | b'#')),
            "Invalid light"
        );
        let (buttons, joltages) = rest
            .rsplit_once(") {")
            .ok_or_eyre("Missing button-joltage seperator")?;
        let joltages: Vec<_> = joltages
            .split(',')
            .map(str::parse)
            .try_collect()
            .wrap_err_with(|| format!("Failed to parse joltage: {joltages:?}"))?;
        ensure!(goal.len() <= 16, "Buttons too long");
        ensure!(goal.len() == joltages.len());
        let goal_len: u32 = goal.len().try_conv()?;

        Self {
            lights: goal.as_bytes().iter().try_fold(0u16, |out, &b| {
                ensure!(matches!(b, b'.' | b'#'), "Invalid light");
                Ok(out << 1 | (u16::from(b) & 1))
            })?,
            buttons: buttons
                .split(") (")
                .map(|button| {
                    button
                        .split(',')
                        .map(u32::from_str)
                        .try_fold(0u16, |out, res| {
                            match res.wrap_err("Failed to parse button") {
                                Ok(bit) if bit < goal_len => (out | (1 << (goal_len - bit - 1)))
                                    .pipe(Ok::<_, color_eyre::Report>),
                                Ok(bit) => bail!("Switch too large ({bit})"),
                                Err(err) => Err(err),
                            }
                        })
                })
                .try_collect()?,
            joltages,
        }
        .pipe(Ok)
    }
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        Self {
            machines: s
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(Machine::from_str)
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
            std::env::VarError::NotPresent => Ok("day_10=debug"),
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
