use day_11::{init_tracing, part1::process};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_tracing()?;
    let puzzle = common::read_input!("input.txt").parse()?;
    let output = process(puzzle);
    println!("{output}");
    Ok(())
}
