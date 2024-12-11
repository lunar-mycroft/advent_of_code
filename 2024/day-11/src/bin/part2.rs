use day_11::{init_tracing, Puzzle};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_tracing()?;
    let puzzle: Puzzle = common::read_input!("part2.txt").parse()?;
    let output = puzzle.breadth_first(75);
    println!("{output}");
    Ok(())
}
