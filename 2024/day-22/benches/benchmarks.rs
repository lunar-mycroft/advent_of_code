use tap::prelude::*;

#[allow(clippy::wildcard_imports)]
use day_22::*;

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
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe(part1::process)
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
                .pipe_ref(part2::initial)
        });
}

#[divan::bench]
fn part2_one_pass(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe_ref(part2::one_pass)
        });
}

#[divan::bench]
fn part2_int_key(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe_ref(part2::u32_key)
        });
}

#[divan::bench]
fn part2_fxhash(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe_ref(part2::fxhash_cache)
        });
}

#[divan::bench]
fn part2_prealloc(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe_ref(part2::pre_alloc)
        });
}

#[divan::bench]
fn part2_btree(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe_ref(part2::btree)
        });
}

#[divan::bench]
fn part2_continuous_windows(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe_ref(part2::continuous_windows)
        });
}

#[divan::bench]
fn part2_vec_cache(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe_ref(part2::vec_cache)
        });
}

#[divan::bench]
fn part2_mul_vec(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe_ref(part2::mul_windows)
        });
}

#[divan::bench]
fn part2_small_cache(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe_ref(part2::small_cache)
        });
}

#[divan::bench]
fn part2_rayon(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe_ref(part2::small_cache_rayon)
        });
}
