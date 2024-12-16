use color_eyre::eyre::OptionExt;
use common::grid::Grid;
use glam::IVec2;
use tap::Pipe;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    map: Grid<u8>,
    start: IVec2,
    end: IVec2,
}

impl Puzzle {
    fn bfs(&self) -> Costs {
        let mut costs = Costs::new(self.map.size());
        assert!(costs.replace_if_lt(self.start, D_0, 0));
        let mut stack = vec![(self.start, D_0, 0)];

        while let Some((pos, dir, cost)) = stack.pop() {
            for (new_p, new_d, new_c) in [
                (pos + dir, dir, cost + 1),
                (pos, dir.perp(), cost + 1000),
                (pos, -dir.perp(), cost + 1000),
            ]
            .into_iter()
            .filter(|(p, _, _)| self.map[*p] != b'#')
            {
                if costs.replace_if_lt(new_p, new_d, new_c) {
                    stack.push((new_p, new_d, new_c));
                }
            }
        }
        costs
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TileCosts {
    x: u32,
    neg_x: u32,
    y: u32,
    neg_y: u32,
}

struct Costs(Grid<TileCosts>);

impl Costs {
    fn new(size: IVec2) -> Self {
        Grid::from_value(
            TileCosts {
                x: u32::MAX,
                neg_x: u32::MAX,
                y: u32::MAX,
                neg_y: u32::MAX,
            },
            size,
        )
        .pipe(Self)
    }
    fn replace_if_lt(&mut self, pos: IVec2, dir: IVec2, cost: u32) -> bool {
        let Some(tile) = self.0.get_mut(pos) else {
            return false;
        };
        match dir {
            IVec2 { x: 1, y: 0 } if tile.x > cost => {
                tile.x = cost;
                true
            }
            IVec2 { x: -1, y: 0 } if tile.neg_x > cost => {
                tile.neg_x = cost;
                true
            }
            IVec2 { x: 0, y: 1 } if tile.y > cost => {
                tile.y = cost;
                true
            }
            IVec2 { x: 0, y: -1 } if tile.neg_y > cost => {
                tile.neg_y = cost;
                true
            }
            _ => false,
        }
    }

    fn get(&self, pos: IVec2, dir: IVec2) -> Option<u32> {
        let tile = self.0.get(pos)?;
        match dir {
            IVec2 { x: 1, y: 0 } => Some(tile.x),
            IVec2 { x: -1, y: 0 } => Some(tile.neg_x),
            IVec2 { x: 0, y: 1 } => Some(tile.y),
            IVec2 { x: 0, y: -1 } => Some(tile.neg_y),
            _ => None,
        }
        .filter(|&v| v < u32::MAX)
    }
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        let map: Grid<u8> = s.parse()?;
        Self {
            start: map
                .positions()
                .zip(map.iter().copied())
                .find_map(|(p, c)| (c == b'S').then_some(p))
                .ok_or_eyre("Missing start")?,
            end: map
                .positions()
                .zip(map.iter().copied())
                .find_map(|(p, c)| (c == b'E').then_some(p))
                .ok_or_eyre("Missing end")?,
            map,
        }
        .pipe(Ok)
    }
}

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];
const D_0: IVec2 = IVec2::X;

pub fn init_tracing() -> color_eyre::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

    let log_filter: EnvFilter = std::env::var(EnvFilter::DEFAULT_ENV)
        .map(String::leak)
        .map(|s| s as &str)
        .or_else(|err| match err {
            std::env::VarError::NotPresent => Ok("day_16=debug"),
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
