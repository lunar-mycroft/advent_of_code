use tap::prelude::*;

#[allow(clippy::wildcard_imports)]
use day_11::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(consts = [
    1,
    10,
    15,
    20,
    25,
    32,
])]
fn simulate_vec<const N: u8>() -> color_eyre::Result<()> {
    let puzzle = common::read_input!("part1.txt")
        .parse::<Puzzle>()?
        .pipe(divan::black_box);
    puzzle.simulate_vec(N);
    Ok(())
}

#[divan::bench(consts = [
    1,
    10,
    15,
    20,
    25,
    32,
])]
fn simulate_math<const N: usize>() -> color_eyre::Result<()> {
    let puzzle = common::read_input!("part1.txt")
        .parse::<Puzzle>()?
        .pipe(divan::black_box);
    let _ = puzzle.simulate_no_alloc(N);
    Ok(())
}

#[divan::bench(consts = [
    1,
    10,
    15,
    20,
    25,
    32,
    40,
    50,
    60,
    70,
    75
])]
fn depth_first_str<const N: u8>() -> color_eyre::Result<()> {
    let puzzle = common::read_input!("part1.txt")
        .parse::<Puzzle>()?
        .pipe(divan::black_box);
    let _ = puzzle.depth_first_str(N);
    Ok(())
}

#[divan::bench(consts = [
    1,
    10,
    15,
    20,
    25,
    32,
    40,
    50,
    60,
    70,
    75
])]
fn depth_first_math<const N: u8>() -> color_eyre::Result<()> {
    let puzzle = common::read_input!("part1.txt")
        .parse::<Puzzle>()?
        .pipe(divan::black_box);
    let _ = puzzle.depth_first_math(N);
    Ok(())
}
