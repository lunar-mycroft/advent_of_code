use color_eyre::eyre::{bail, OptionExt, Result};
use glam::IVec2;
use itertools::Itertools;
use tap::prelude::*;

use common::grid::Grid;

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub struct Puzzle {
    map: Grid<u8>,
    moves: Vec<IVec2>,
    bot: IVec2,
}

impl Puzzle {
    fn wide(self) -> Self {
        let tiles = self
            .map
            .iter()
            .copied()
            .flat_map(|b| match b {
                b'#' => b"##",
                b'.' => b"..",
                b'O' => b"[]",
                b'@' => b"@.",
                b'[' | b']' => panic!("Attempted to widden twice"),
                _ => unreachable!(),
            })
            .copied();
        let size = self.map.size();
        Self {
            map: Grid::from_row_major_ordered(
                tiles,
                IVec2 {
                    x: size.x * 2,
                    y: size.y,
                },
            ),
            moves: self.moves,
            bot: IVec2 {
                x: self.bot.x * 2,
                y: self.bot.y,
            },
        }
    }

    #[allow(unused)]
    fn print_grid(&self) -> Result<()> {
        let width = self.map.size().x.try_conv::<usize>()?;
        for chunk in &self.map.iter().copied().chunks(width) {
            let s = chunk
                .map(u32::from)
                .filter_map(char::from_u32)
                .collect::<String>();
            println!("{s}");
        }
        Ok(())
    }

    fn can_move(&self, pos: IVec2, dir: IVec2) -> bool {
        let thing = *self.map.get(pos).expect("pos outside of map");
        match self.map.get(pos + dir).copied() {
            _ if thing == b'.' => false,
            Some(b'.') => true,
            Some(b'#') => false,
            Some(b'O') => self.can_move(pos + dir, dir),
            Some(b'[' | b']') if dir.y == 0 => self.can_move(pos + dir, dir),
            Some(b'[') if dir.x == 0 => {
                self.can_move(pos + dir, dir) && self.can_move(pos + dir + IVec2::X, dir)
            }
            Some(b']') if dir.x == 0 => {
                self.can_move(pos + dir, dir) && self.can_move(pos + dir - IVec2::X, dir)
            }
            Some(b'@') => panic!("Attempted to move into robot"),
            Some(b) => unreachable!("{:?}", b.conv::<u32>().pipe(char::from_u32)),
            None => panic!("Attempted to move outside of map"),
        }
    }

    fn move_unchecked(&mut self, pos: IVec2, dir: IVec2) {
        let thing = *self.map.get(pos).expect("pos outside of map");
        if thing == b'@' {
            self.bot = pos + dir;
        }
        *self.map.get_mut(pos + dir).expect("known inside") = thing;
        *self.map.get_mut(pos).expect("known inside") = b'.';
    }

    fn step(&mut self, pos: IVec2, dir: IVec2) -> bool {
        debug_assert_eq!((dir.abs().max_element(), dir.abs().min_element()), (1, 0));
        match self.map.get(pos + dir).copied() {
            _ if !self.can_move(pos, dir) => false,
            Some(b'.') => {
                self.move_unchecked(pos, dir);
                true
            }
            Some(b'O') => {
                assert!(self.step(pos + dir, dir));
                self.move_unchecked(pos, dir);
                true
            }
            Some(b'#') => false,
            Some(b'@') => panic!("Attempted to move into robot"),
            Some(b'[' | b']') if dir.y == 0 => {
                assert!(self.step(pos + dir, dir));
                self.move_unchecked(pos, dir);
                true
            }
            Some(b'[') => {
                assert!(self.step(pos + dir, dir) && self.step(pos + dir + IVec2::X, dir));
                self.move_unchecked(pos, dir);
                true
            }
            Some(b']') => {
                assert!(self.step(pos + dir, dir) && self.step(pos + dir - IVec2::X, dir));
                self.move_unchecked(pos, dir);
                true
            }
            Some(_) => unreachable!(),
            None => panic!("Attempted to move outside of map"),
        }
    }

    fn sum_coords(&self) -> i32 {
        self.map
            .positions()
            .zip(self.map.iter().copied())
            .filter_map(|(pos, thing)| matches!(thing, b'O' | b'[').then_some(pos))
            .map(|pos| pos.y * 100 + pos.x)
            .sum()
    }
}

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        let s = s.replace('\r', "");
        let (map, moves) = s
            .split_once("\n\n")
            .ok_or_eyre("Couldn't seperate map and moves")?;
        let map: Grid<u8> = map.parse()?;

        Self {
            bot: map
                .positions()
                .zip(map.iter().copied())
                .find_map(|(pos, thing)| (thing == b'@').then_some(pos))
                .ok_or_eyre("Missing bot")?,
            map,
            moves: moves
                .trim()
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| match c {
                    '^' => Ok(-IVec2::Y),
                    '>' => Ok(IVec2::X),
                    'v' => Ok(IVec2::Y),
                    '<' => Ok(-IVec2::X),
                    c => bail!("Unexpected character {c:?}"),
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
            std::env::VarError::NotPresent => Ok("day_15=debug"),
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
