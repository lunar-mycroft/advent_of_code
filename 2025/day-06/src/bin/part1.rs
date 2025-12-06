use day_06::{init_tracing, part1::process, Puzzle};
use tap::Pipe;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_tracing()?;
    let puzzle = common::read_input!("part1.txt").pipe_deref(Puzzle::parse_part_1)?;
    let output = process(puzzle);
    println!("{output}");
    Ok(())
}
