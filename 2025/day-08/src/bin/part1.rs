use day_08::{init_tracing, part1::process, Puzzle};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_tracing()?;
    let puzzle: Puzzle = common::read_input!("part1.txt").parse()?;
    let by_distance = puzzle.by_distance();
    let output = process((puzzle, by_distance), 1000);
    println!("{output}");
    Ok(())
}
