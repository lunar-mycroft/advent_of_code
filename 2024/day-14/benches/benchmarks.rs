use glam::IVec2;
use tap::prelude::*;

#[allow(clippy::wildcard_imports)]
use day_14::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn parse(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| common::read_input!("part1.txt").pipe(Ok::<_, color_eyre::Report>))
        .bench_values(|res| res.expect("file to be loaded").parse::<Puzzle>());
}

#[divan::bench]
fn part1(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| common::read_input!("part1.txt").parse::<Puzzle>())
        .bench_values(|res| {
            let puzzle = res.expect("parsing to suceed").pipe(divan::black_box);
            part1::process(puzzle, IVec2::new(101, 103))
        });
}

#[divan::bench]
fn part2_hashmap(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            let puzzle = res.expect("parsing to suceed").pipe(divan::black_box);
            part2::process(puzzle, IVec2::new(101, 103))
        });
}

#[divan::bench]
fn part2_grid_unique(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            let puzzle = res.expect("parsing to suceed").pipe(divan::black_box);
            part2::process_grid_unique(puzzle, IVec2::new(101, 103))
        });
}
