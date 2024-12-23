use day_23::{init_tracing, part1::initial};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_tracing()?;
    let puzzle = common::read_input!("part1.txt").parse()?;
    let output = initial(&puzzle);
    println!("{output}");
    Ok(())
}
