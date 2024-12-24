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
            .with_inputs(|| common::read_input!("part1.txt").parse::<StringGraph>())
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe_ref(part1::initial)
            });
    }

    #[divan::bench]
    fn common(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| common::read_input!("part1.txt").parse::<StringGraph>())
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe_ref(part1::common_methods)
            });
    }

    #[divan::bench]
    fn edge_set(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| common::read_input!("part1.txt").parse::<StringGraph>())
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe_ref(part1::edge_set)
            });
    }

    #[divan::bench]
    fn pre_filter(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| common::read_input!("part1.txt").parse::<StringGraph>())
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe_ref(part1::pre_filter)
            });
    }

    #[divan::bench]
    fn int_graph(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| common::read_input!("part1.txt").parse::<IntGraph>())
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe_ref(part1::int_graph)
            });
    }

    #[divan::bench]
    fn array(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| common::read_input!("part1.txt").parse::<IntGraph>())
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe_ref(part1::array)
            });
    }

    #[divan::bench]
    fn array_pre_parsed(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| {
                common::read_input!("part1.txt")
                    .parse::<IntGraph>()
                    .map(|p| array::parse(&p))
            })
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe_ref(part1::array_preparsed)
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
            .with_inputs(|| common::read_input!("part2.txt").parse::<StringGraph>())
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe_ref(part2::initial)
            });
    }

    #[divan::bench]
    fn common(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| common::read_input!("part2.txt").parse::<StringGraph>())
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe_ref(part2::common_methods)
            });
    }

    #[divan::bench]
    fn bron_kerbosh(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| common::read_input!("part2.txt").parse::<StringGraph>())
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe_ref(part2::bron_kerbosh)
            });
    }

    #[divan::bench]
    fn fx_hash(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| common::read_input!("part2.txt").parse::<StringGraph>())
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe_ref(part2::fx_hash)
            });
    }

    #[divan::bench]
    fn int_graph(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| common::read_input!("part2.txt").parse::<IntGraph>())
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe_ref(part2::int_graph)
            });
    }

    #[divan::bench]
    fn array(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| common::read_input!("part2.txt").parse::<IntGraph>())
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe_ref(part2::array)
            });
    }

    #[divan::bench]
    fn array_inline(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| common::read_input!("part2.txt").parse::<IntGraph>())
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe_ref(part2::array_inline)
            });
    }

    #[divan::bench]
    fn array_pre_parsed(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| {
                common::read_input!("part2.txt")
                    .parse::<IntGraph>()
                    .map(|p| array::parse(&p))
            })
            .bench_values(|res| {
                res.expect("parsing to suceed")
                    .pipe(divan::black_box)
                    .pipe_ref(part2::array_preparsed)
            });
    }
}

#[allow(clippy::wildcard_imports)]
#[divan::bench_group]
mod parse {
    use super::*;

    #[divan::bench]
    fn string_repr(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| common::read_input!("part1.txt").pipe(Ok::<_, color_eyre::Report>))
            .bench_values(|res| res.expect("file to be loaded").parse::<StringGraph>());
    }

    #[divan::bench]
    fn int_repr(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| common::read_input!("part1.txt").pipe(Ok::<_, color_eyre::Report>))
            .bench_values(|res| res.expect("file to be loaded").parse::<IntGraph>());
    }

    #[divan::bench]
    fn to_array(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| common::read_input!("part1.txt").parse::<IntGraph>())
            .bench_values(|res| res.expect("parsing to succeed").pipe_ref(array::parse));
    }
}
