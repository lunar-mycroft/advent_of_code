use tap::prelude::*;

#[allow(clippy::wildcard_imports)]
use day_23::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[allow(clippy::wildcard_imports)]
#[divan::bench_group]
mod part_1 {
    use super::*;

    #[divan::bench]
    fn initial(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| common::read_input!("part1.txt").parse::<Puzzle>())
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe_ref(part1::initial)
            });
    }

    #[divan::bench]
    fn common(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| common::read_input!("part1.txt").parse::<Puzzle>())
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe_ref(part1::common_methods)
            });
    }
}

#[allow(clippy::wildcard_imports)]
#[divan::bench_group]
mod part_2 {
    use super::*;

    #[divan::bench]
    fn initial(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| common::read_input!("part2.txt").parse::<Puzzle>())
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe_ref(part2::initial)
            });
    }

    #[divan::bench]
    fn common(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| common::read_input!("part2.txt").parse::<Puzzle>())
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe_ref(part2::common_methods)
            });
    }
}

#[divan::bench]
fn parse(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| common::read_input!("part1.txt").pipe(Ok::<_, color_eyre::Report>))
        .bench_values(|res| res.expect("file to be loaded").parse::<Puzzle>());
}
