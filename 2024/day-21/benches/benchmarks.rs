use tap::prelude::*;

#[allow(clippy::wildcard_imports)]
use day_21::*;

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

#[divan::bench_group]
mod initial {
    use tap::prelude::*;

    use day_21::Puzzle;

    #[divan::bench]
    fn part1(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| common::read_input!("part1.txt").parse::<Puzzle>())
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe(day_21::initial::part1)
            });
    }

    #[divan::bench]
    fn part2(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| {
                common::read_input!("part2.txt")
                    .parse::<Puzzle>()
                    .map(divan::black_box)
            })
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe(day_21::initial::part2)
            });
    }
}

#[divan::bench(args = [2, 25])]
fn generalized(bencher: divan::Bencher, layers: u8) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            let puzzle = res.expect("parsing to suceed").pipe(divan::black_box);
            generalized::process(&puzzle, layers);
        });
}

#[divan::bench(args = [2, 25])]
fn idomatic(bencher: divan::Bencher, layers: u8) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            let puzzle = res.expect("parsing to suceed").pipe(divan::black_box);
            idiomatic::process(&puzzle, layers)
        });
}
