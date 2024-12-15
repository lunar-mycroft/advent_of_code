use color_eyre::Result;
use glam::IVec2;

use day_14::{init_tracing, Puzzle};

fn main() -> Result<()> {
    init_tracing()?;
    let puzzle: Puzzle = common::read_input!("part1.txt").parse()?;
    puzzle
        .render(IVec2 { x: 101, y: 103 }, 6668)
        .save("test.png")?;
    Ok(())
}
