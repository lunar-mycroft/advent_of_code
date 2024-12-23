use day_24::{init_tracing, part2::process};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_tracing()?;
    let puzzle = common::read_input!("part1.txt").parse()?;
    let output = process(&puzzle).expect("to find a solution");
    println!("{output}");
    Ok(())
}
