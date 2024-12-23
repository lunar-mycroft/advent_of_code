use day_18::{init_tracing, part2::process_astar_rev};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_tracing()?;
    let puzzle = common::read_input!("part1.txt").parse()?;
    let output = process_astar_rev(puzzle);
    println!("{output}");
    Ok(())
}
