use tap::prelude::*;

#[allow(clippy::wildcard_imports)]
use day_13::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(sample_size = 100)]
fn part1_matrix() -> color_eyre::Result<()> {
    common::read_input!("part1.txt")
        .parse::<Puzzle>()?
        .pipe(divan::black_box)
        .pipe(part1::process_mat);
    Ok(())
}

#[divan::bench(sample_size = 100)]
fn part1_integer() -> color_eyre::Result<()> {
    common::read_input!("part1.txt")
        .parse::<Puzzle>()?
        .pipe(divan::black_box)
        .pipe(part1::process_int);
    Ok(())
}

#[divan::bench(sample_size = 100)]
fn part2_matrix() -> color_eyre::Result<()> {
    common::read_input!("part2.txt")
        .parse::<Puzzle>()?
        .pipe(divan::black_box)
        .pipe(part2::process_mat);
    Ok(())
}

#[divan::bench(sample_size = 100)]
fn part2_integer() -> color_eyre::Result<()> {
    common::read_input!("part2.txt")
        .parse::<Puzzle>()?
        .pipe(divan::black_box)
        .pipe(part2::process_int);
    Ok(())
}
