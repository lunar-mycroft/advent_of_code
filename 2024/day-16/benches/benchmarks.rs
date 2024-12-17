use tap::prelude::*;

#[allow(clippy::wildcard_imports)]
use day_16::*;

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
fn part1_bfs(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| common::read_input!("part1.txt").parse::<Puzzle>())
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe(part1::bfs)
        });
}

#[divan::bench]
fn part1_dijkstras(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| common::read_input!("part1.txt").parse::<Puzzle>())
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe(part1::dijkstras)
        });
}

#[divan::bench]
fn part1_astar(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| common::read_input!("part1.txt").parse::<Puzzle>())
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe(part1::astar)
        });
}

#[divan::bench]
fn part2_bfs(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe(part2::bfs)
        });
}

#[divan::bench]
fn part2_dijkstras(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| common::read_input!("part2.txt").parse::<Puzzle>())
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe(part2::dijkstras)
        });
}

#[divan::bench]
fn part2_astar(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| common::read_input!("part2.txt").parse::<Puzzle>())
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe(part2::astar)
        });
}
