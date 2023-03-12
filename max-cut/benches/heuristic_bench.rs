use std::{fs, fmt::format, time::Duration};

use criterion::{criterion_main, criterion_group, Criterion, BenchmarkId, PlotConfiguration};
use max_cut::*;

pub fn heuristic_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("heuristic_bench");

    let graphs = fs::read_dir("benches/data").unwrap();

    for graph_path in graphs {
        let graph_path = graph_path.unwrap().path();
        let raw = fs::read_to_string(&graph_path).unwrap();
        let graph = graph_parser::parse_graph(&raw).unwrap();

        group.throughput(criterion::Throughput::Elements(graph.size() as u64));
        group.sample_size(10);
        group.measurement_time(Duration::from_millis(1));
        group.warm_up_time(Duration::from_millis(1));
        group.plot_config(PlotConfiguration::default());
        group.bench_with_input(BenchmarkId::new("Single threaded", graph_path.file_name().unwrap().to_str().unwrap()), &graph, |b, g| {
            b.iter(|| heuristic::rand_aprox(g))
        });
        group.bench_with_input(BenchmarkId::new("Multi threaded", graph_path.file_name().unwrap().to_str().unwrap()), &graph, |b, g| {
            b.iter(|| heuristic::rand_aprox_parallel(g))
        });
    }

    group.finish();
}

criterion_group!(benches, heuristic_bench);
criterion_main!(benches);
