use tap::prelude::*;

#[allow(clippy::wildcard_imports)]
use day_08::*;

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
fn sort(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part1.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|puzzle| puzzle.expect("parsing to succeed").by_distance());
}

#[divan::bench]
fn part1(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let puzzle: Puzzle = common::read_input!("part1.txt").parse()?;
            let by_distance = puzzle.by_distance();
            (puzzle, by_distance)
                .pipe(divan::black_box)
                .pipe(Ok::<_, color_eyre::Report>)
        })
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe(|input| part1::process(input, 1000))
        });
}

#[divan::bench]
fn part2(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let puzzle: Puzzle = common::read_input!("part1.txt").parse()?;
            let by_distance = puzzle.by_distance();
            (puzzle, by_distance)
                .pipe(divan::black_box)
                .pipe(Ok::<_, color_eyre::Report>)
        })
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe(part2::process)
        });
}
