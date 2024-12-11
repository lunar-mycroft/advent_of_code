use tap::prelude::*;

#[allow(clippy::wildcard_imports)]
use day_11::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() -> color_eyre::Result<()> {
    common::read_input!("part1.txt")
        .parse::<Puzzle>()?
        .pipe(divan::black_box)
        .pipe(part1::process);
    Ok(())
}

#[divan::bench]
fn part1_sum() -> color_eyre::Result<()> {
    common::read_input!("part1.txt")
        .parse::<Puzzle>()?
        .pipe(divan::black_box)
        .pipe(part1::process_sum);
    Ok(())
}

#[divan::bench]
fn part2() -> color_eyre::Result<()> {
    common::read_input!("part2.txt")
        .parse::<Puzzle>()?
        .pipe(divan::black_box)
        .pipe(part2::process);
    Ok(())
}

#[divan::bench]
fn part2_str_free() -> color_eyre::Result<()> {
    common::read_input!("part2.txt")
        .parse::<Puzzle>()?
        .pipe(divan::black_box)
        .pipe(part2::process_str_free);
    Ok(())
}
