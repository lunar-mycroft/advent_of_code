use color_eyre::eyre::{ensure, OptionExt};
use glam::IVec2;
use itertools::Itertools;
use tap::{Pipe, TryConv};

pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
pub struct Puzzle {
    width: usize,
    heights: Vec<u8>,
}

impl Puzzle {
    fn size(&self) -> IVec2 {
        IVec2 {
            x: self.width.try_into().expect("size to fit in i32"),
            y: (self.heights.len() / self.width)
                .try_into()
                .expect("size to fit in i32"),
        }
    }

    fn get(&self, pos: IVec2) -> Option<u8> {
        let size = self.size();
        if pos.min_element() < 0 || pos.x >= size.x || pos.y >= size.y {
            return None;
        }
        let idx = pos.y.try_conv::<usize>().expect("!") * self.width
            + pos.x.try_conv::<usize>().expect("!");
        self.heights.get(idx).copied()
    }

    fn iter(&self) -> impl Iterator<Item = (IVec2, u8)> + '_ {
        let size = self.size();
        (0..size.y).cartesian_product(0..size.x).map(|(y, x)| {
            let pos = IVec2 { x, y };
            (pos, self.get(pos).expect("known in range"))
        })
    }
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        ensure!(
            s.lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(str::len)
                .all_equal(),
            "Unequal lines"
        );
        let width = s.lines().next().ok_or_eyre("No lines in input")?.len();
        let height = s.lines().count();
        let heights = s
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| u8::try_from(d).expect("9 < 255"))
            .collect_vec();
        ensure!(width * height == heights.len(), "Dimention mismatch");
        Self { width, heights }.pipe(Ok)
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
