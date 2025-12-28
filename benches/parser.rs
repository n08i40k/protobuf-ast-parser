use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn parse_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");
    let cases = [
        (
            "benchmark0",
            include_str!("../proto/bench/benchmark0.proto"),
        ),
        (
            "benchmark1",
            include_str!("../proto/bench/benchmark1.proto"),
        ),
        (
            "benchmark2",
            include_str!("../proto/bench/benchmark2.proto"),
        ),
        (
            "benchmark3",
            include_str!("../proto/bench/benchmark3.proto"),
        ),
        (
            "benchmark4",
            include_str!("../proto/bench/benchmark4.proto"),
        ),
        (
            "benchmark5",
            include_str!("../proto/bench/benchmark5.proto"),
        ),
    ];

    for (name, source) in cases {
        group.bench_with_input(BenchmarkId::new("proto", name), source, |b, data| {
            b.iter(|| {
                let ast =
                    protobuf_ast_parser::parse(std::hint::black_box(data)).expect("valid proto");
                std::hint::black_box(ast);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, parse_bench);
criterion_main!(benches);
