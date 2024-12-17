use color_eyre::eyre::OptionExt;
use itertools::Itertools;
use tap::prelude::*;

pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
pub struct Puzzle {
    a: u64,
    b: u64,
    c: u64,
    program_counter: usize,
    program: Vec<u8>,
}

impl Iterator for Puzzle {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            assert_eq!(self.program_counter % 2, 0);
            match self
                .program
                .get(self.program_counter..=self.program_counter + 1)
            {
                Some([0, x]) => {
                    self.a /= 1 << self.combo(*x);
                    self.program_counter += 2;
                }
                Some([1, x]) => {
                    self.b ^= u64::from(*x);
                    self.program_counter += 2;
                }
                Some([2, x]) => {
                    self.b = self.combo(*x) % 8;
                    self.program_counter += 2;
                }
                Some([3, x]) => {
                    if self.a != 0 {
                        self.program_counter = usize::from(*x);
                    } else {
                        self.program_counter += 2;
                    }
                }
                Some([4, _]) => {
                    self.b ^= self.c;
                    self.program_counter += 2;
                }
                Some([5, x]) => {
                    self.program_counter += 2;
                    break Some((self.combo(*x) % 8) as u8);
                }
                Some([6, x]) => {
                    self.b = self.a / (1 << self.combo(*x));
                    self.program_counter += 2;
                }
                Some([7, x]) => {
                    self.c = self.a / (1 << self.combo(*x));
                    self.program_counter += 2;
                }
                None | Some([_]) => break None,
                _ => unreachable!(),
            }
        }
    }
}

impl Puzzle {
    #[cfg(test)]
    fn run(&mut self) -> Vec<u8> {
        let mut out = Vec::new();
        for n in self.by_ref() {
            out.push(n);
        }
        out
    }

    fn combo(&self, x: u8) -> u64 {
        match x {
            x @ 0..=3 => x.into(),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid operand {x}"),
        }
    }
}

// enum Instruction {
//     Adv(u8), // opcode 0, combo,   A = A / 2^x
//     Bxl(u8), // opcode 1, literal, B = B ^ x
//     Bst(u8), // opcode 2, combo,   B = x & 0b111
//     Jnz(u8), //
// }

impl std::str::FromStr for Puzzle {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> color_eyre::Result<Self> {
        let (reg_a, reg_b, reg_c, instructions) = s
            .lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.split_once(": "))
            .map(|(_, s)| s)
            .collect_tuple()
            .ok_or_eyre("Incorrect number of lines")?;
        Self {
            a: reg_a.parse()?,
            b: reg_b.parse()?,
            c: reg_c.parse()?,
            program_counter: 0,
            program: instructions.split(',').map(str::parse).try_collect()?,
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
            std::env::VarError::NotPresent => Ok("day_17=debug"),
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
