use color_eyre::eyre::ensure;
use itertools::Itertools;
use tap::Pipe;

pub mod part1;
pub mod part2;

pub mod generalized;
pub mod idiomatic;
pub mod initial;
pub mod no_hash;

#[derive(Debug, Clone)]
pub struct Puzzle {
    codes: Vec<(usize, String)>,
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        Self {
            codes: s
                .trim()
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|line| {
                    ensure!(line.is_ascii(), "Line not ascii");
                    ensure!(line.len() == 4, "Line wrong length");
                    (line[..3].parse()?, line.to_owned()).pipe(Ok::<_, color_eyre::Report>)
                })
                .try_collect()?,
        }
        .pipe(Ok)
    }
}

// #[derive(Debug, Clone)]
// struct Counter(HashMap<String, usize>);

// impl Counter {
//     fn update(&mut self, other: Counter) {
//         for (key, value) in other.0 {
//             *self.0.entry(key).or_insert(0) += value;
//         }
//     }

//     fn total_len(&self) -> usize {
//         self.0.iter().map(|(k, v)| k.len() * v).sum::<usize>()
//     }
// }

// impl FromIterator<String> for Counter {
//     fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
//         iter.into_iter()
//             .fold(HashMap::new(), |mut map, s| {
//                 *map.entry(s).or_insert(0) += 1;
//                 map
//             })
//             .pipe(Self)
//     }
// }

pub fn init_tracing() -> color_eyre::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

    let log_filter: EnvFilter = std::env::var(EnvFilter::DEFAULT_ENV)
        .map(String::leak)
        .map(|s| s as &str)
        .or_else(|err| match err {
            std::env::VarError::NotPresent => Ok("day_21=debug"),
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
