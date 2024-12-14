use day_14::{init_tracing, part2::process};
use glam::IVec2;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_tracing()?;
    let puzzle = common::read_input!("part1.txt").parse()?;
    let output = process(puzzle, IVec2 { x: 101, y: 103 });
    println!("{output}");
    Ok(())
}
