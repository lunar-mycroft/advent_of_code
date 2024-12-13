use tap::prelude::*;

#[allow(clippy::wildcard_imports)]
use {{crate_name}}::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part1.txt")
                .parse::<Puzzle>()
        })
        .bench_values(|res| res.expect("parsing to suceed").pipe(divan::black_box).pipe(part1::process));
}

#[divan::bench]
fn part1_integer(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| res.expect("parsing to suceed").pipe(divan::black_box).pipe(part2::process));
}
