extern crate criterion;
extern crate inferno;

use criterion::*;
use inferno::flamegraph::{self, Options};
use std::fs::File;
use std::io::{self, BufReader, Read};

fn flamegraph_benchmark(c: &mut Criterion, id: &str, infile: &str, mut opt: Options<'static>) {
    let mut f = File::open(infile).expect("file not found");

    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes).expect("Could not read file");

    c.bench(
        "flamegraph",
        ParameterizedBenchmark::new(
            id,
            move |b, data| {
                b.iter(|| {
                    let reader = BufReader::new(data.as_slice());
                    let _folder = flamegraph::from_reader(&mut opt, reader, io::sink());
                })
            },
            vec![bytes],
        )
        .throughput(|bytes| Throughput::Bytes(bytes.len() as u32)),
    );
}

macro_rules! flamegraph_benchmarks {
    ($($name:ident : ($infile:expr, $opt:expr)),*) => {
        $(
            fn $name(c: &mut Criterion) {
                let id = stringify!($name);
                flamegraph_benchmark(c, id, $infile, $opt);
            }
        )*

        criterion_group!(benches, $($name),*);
        criterion_main!(benches);
    }
}

flamegraph_benchmarks! {
    dtrace: ("tests/data/collapse-dtrace/results/dtrace-example.txt", Default::default()),
    perf: ("tests/data/collapse-perf/results/example-perf-stacks-collapsed.txt", Default::default()),
    dtrace_reverse: ("tests/data/collapse-dtrace/results/dtrace-example.txt",
                     Options { reverse_stack_order: true, ..Default::default() }),
    perf_reverse: ("tests/data/collapse-perf/results/example-perf-stacks-collapsed.txt",
                     Options { reverse_stack_order: true, ..Default::default() })
}
