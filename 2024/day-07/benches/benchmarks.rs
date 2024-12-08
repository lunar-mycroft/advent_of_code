use tap::prelude::*;

#[allow(clippy::wildcard_imports)]
use day_07::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() -> color_eyre::Result<()> {
    common::read_input!("part1.txt")
        .pipe(divan::black_box)
        .pipe_deref(part1::process)?;
    Ok(())
}

#[divan::bench]
fn part1_rayon() -> color_eyre::Result<()> {
    common::read_input!("part1.txt")
        .pipe(divan::black_box)
        .pipe_deref(part1::process_rayon)?;
    Ok(())
}

#[divan::bench]
fn part2() -> color_eyre::Result<()> {
    common::read_input!("part2.txt")
        .pipe(divan::black_box)
        .pipe_deref(part2::process)?;
    Ok(())
}

#[divan::bench]
fn part2_rayyon() -> color_eyre::Result<()> {
    common::read_input!("part2.txt")
        .pipe(divan::black_box)
        .pipe_deref(part2::process_rayon)?;
    Ok(())
}
