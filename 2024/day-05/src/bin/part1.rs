use day_05::{init_tracing, part1::process};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_tracing()?;
    let input = common::read_input!("part1.txt");
    let output = process(&input)?;
    println!("{output}");
    Ok(())
}
