use day_13::{init_tracing, part1::process_mat};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_tracing()?;
    let puzzle = common::read_input!("part1.txt").parse()?;
    let output = process_mat(puzzle);
    println!("{output}");
    Ok(())
}
