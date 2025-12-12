use day_12::{init_tracing, process::process};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_tracing()?;
    let puzzle = common::read_input!("input.txt").parse()?;
    let output = process(puzzle);
    println!("{output}");
    Ok(())
}
