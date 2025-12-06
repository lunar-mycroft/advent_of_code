use tap::prelude::*;

use day_06::{init_tracing, part2::process, Puzzle};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_tracing()?;
    let puzzle = common::read_input!("part2.txt").pipe_deref(Puzzle::parse_part_2)?;
    let output = process(puzzle);
    println!("{output}");
    Ok(())
}
