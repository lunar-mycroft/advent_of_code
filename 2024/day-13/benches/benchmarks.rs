use tap::prelude::*;

#[allow(clippy::wildcard_imports)]
use day_13::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(sample_count = 10_000)]
fn part1_matrix(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part1.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| res.expect("parsing to suceed").pipe(part1::process_mat));
}

#[divan::bench(sample_count = 10_000)]
fn part1_integer(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part1.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| res.expect("parsing to suceed").pipe(part1::process_int));
}

#[divan::bench(sample_count = 10_000)]
fn part2_matrix(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| res.expect("parsing to suceed").pipe(part2::process_mat));
}

#[divan::bench(sample_count = 10_000)]
fn part2_integer(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| res.expect("parsing to suceed").pipe(part2::process_int));
}
