use tap::prelude::*;

#[allow(clippy::wildcard_imports)]
use day_19::*;

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
                .pipe(part2::process)
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
                .pipe(part2::process_vec_cache)
        });
}

#[divan::bench]
fn part2_hash_trie_recursive(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe_ref(hash_trie::process_recursive)
        });
}

#[divan::bench]
fn part2_hash_trie_loop(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe_ref(hash_trie::process_loop)
        });
}

#[divan::bench]
fn part2_hash_trie_on_stack(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe_ref(hash_trie::process_loop_on_stack)
        });
}

#[divan::bench(sample_size = 1000)]
fn part2_array_trie_big(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe_ref(array_trie::process_big)
        });
}

#[divan::bench(sample_size = 1000)]
fn part2_array_trie_small(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            common::read_input!("part2.txt")
                .parse::<Puzzle>()
                .map(divan::black_box)
        })
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe_ref(array_trie::process_small)
        });
}

#[divan::bench(sample_size = 1000)]
fn part2_array_trie_no_parse(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| common::read_input!("part2.txt").pipe(Ok::<_, color_eyre::Report>))
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe_deref(array_trie::process_no_parse)
                .expect("to compute an answer");
        });
}

#[divan::bench(sample_size = 1000)]
fn part2_array_trie_inline_partial(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| common::read_input!("part2.txt").pipe(Ok::<_, color_eyre::Report>))
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe_deref(array_trie::process_partial_inline)
                .expect("to compute an answer");
        });
}

#[divan::bench(sample_size = 1000)]
fn part2_array_trie_inline_full(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| common::read_input!("part2.txt").pipe(Ok::<_, color_eyre::Report>))
        .bench_values(|res| {
            res.expect("parsing to suceed")
                .pipe(divan::black_box)
                .pipe_deref(array_trie::process_fully_inline)
                .expect("to compute an answer");
        });
}
