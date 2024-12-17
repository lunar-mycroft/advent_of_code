use day_16::{init_tracing, part2::bfs};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_tracing()?;
    let puzzle = common::read_input!("part1.txt").parse()?;
    let output = bfs(puzzle);
    println!("{output}");
    Ok(())
}
