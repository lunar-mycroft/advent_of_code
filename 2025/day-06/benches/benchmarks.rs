use tap::prelude::*;

#[allow(clippy::wildcard_imports)]
use day_06::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn parse_part_1(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| common::read_input!("part1.txt").pipe(Ok::<_, color_eyre::Report>))
        .bench_values(|res| {
            res.expect("file to be loaded")
                .pipe_deref(Puzzle::parse_part_1)
        });
}

#[divan::bench]
fn parse_part_2(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| common::read_input!("part1.txt").pipe(Ok::<_, color_eyre::Report>))
        .bench_values(|res| {
            res.expect("file to be loaded")
                .pipe_deref(Puzzle::parse_part_2)
        });
}

#[divan::bench]
fn part1(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| common::read_input!("part1.txt").pipe_deref(Puzzle::parse_part_1))
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe(part1::process)
        });
}

#[divan::bench]
fn part2(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| common::read_input!("part2.txt").pipe_deref(Puzzle::parse_part_2))
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe(part2::process)
        });
}
