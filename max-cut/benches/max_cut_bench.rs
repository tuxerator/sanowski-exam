use std::fs;

use criterion::{
    criterion_group, criterion_main,
    BenchmarkId, Criterion, PlotConfiguration, AxisScale,
};
use max_cut::*;

pub fn approx_bench(c: &mut Criterion) {
    let mut time_group = c.benchmark_group("approx_bench");

    let graphs = fs::read_dir("benches/data/vc_exact").unwrap();

    for graph_path in graphs {
        let graph_path = graph_path.unwrap().path();
        let raw = fs::read_to_string(&graph_path).unwrap();
        let graph = graph_parser::parse_pace_graph(&raw).unwrap();

        let id = format!(
            "graph: {}, vetices: {}, edges: {}",
            graph_path.file_name().unwrap().to_str().unwrap(),
            graph.size(),
            graph.edge_size()
        );

        time_group.throughput(criterion::Throughput::Elements((graph.size() + graph.edge_size()) as u64));
        time_group.sample_size(20);
        // time_group.measurement_time(Duration::from_millis(1));
        // time_group.warm_up_time(Duration::from_millis(1));
        time_group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
        time_group.bench_with_input(BenchmarkId::new("approx basic", &id), &graph, |b, g| {
            b.iter(|| approx::max_cut_greedy(g))
        });
        time_group.bench_with_input(BenchmarkId::new("approx improved", &id), &graph, |b, g| {
            b.iter(|| approx::max_cut_greedy_impr(g))
        });
    }

    time_group.finish();
}

pub fn heuristic_bench(c: &mut Criterion) {
    let mut time_group = c.benchmark_group("heuristic_bench");

    let graphs = fs::read_dir("benches/data/vc_exact").unwrap();

    for graph_path in graphs {
        let graph_path = graph_path.unwrap().path();
        let raw = fs::read_to_string(&graph_path).unwrap();
        let graph = graph_parser::parse_pace_graph(&raw).unwrap();

        let id = format!(
            "graph: {}, vetices: {}, edges: {}",
            graph_path.file_name().unwrap().to_str().unwrap(),
            graph.size(),
            graph.edge_size()
        );

        time_group.throughput(criterion::Throughput::Elements((graph.size() + graph.edge_size()) as u64));
        time_group.sample_size(20);
        // time_group.measurement_time(Duration::from_millis(1));
        // time_group.warm_up_time(Duration::from_millis(1));
        time_group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
        time_group.bench_with_input(BenchmarkId::new("heuristic_basic", &id), &graph, |b, g| {
            b.iter(|| heuristic::rand_aprox(g))
        });
        time_group.bench_with_input(BenchmarkId::new("heuristic_parallel", &id), &graph, |b, g| {
            b.iter(|| heuristic::rand_aprox_parallel(g))
        });
    }

    time_group.finish();
}

pub fn heuristic_approx_bench(c: &mut Criterion) {
    let mut time_group = c.benchmark_group("heuristic_approx_bench");

    let graphs = fs::read_dir("benches/data/vc_exact").unwrap();

    for graph_path in graphs {
        let graph_path = graph_path.unwrap().path();
        let raw = fs::read_to_string(&graph_path).unwrap();
        let graph = graph_parser::parse_pace_graph(&raw).unwrap();

        let id = format!(
            "graph: {}, vetices: {}, edges: {}",
            graph_path.file_name().unwrap().to_str().unwrap(),
            graph.size(),
            graph.edge_size()
        );

        time_group.throughput(criterion::Throughput::Elements((graph.size() + graph.edge_size()) as u64));
        time_group.sample_size(20);
        // time_group.measurement_time(Duration::from_secs(1));
        // time_group.warm_up_time(Duration::from_millis(1));
        time_group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
        time_group.bench_with_input(BenchmarkId::new("heuristic_parallel", &id), &graph, |b, g| {
            b.iter(|| heuristic::rand_aprox(g))
        });
        time_group.bench_with_input(BenchmarkId::new("approx_improved", &id), &graph, |b, g| {
            b.iter(|| approx::max_cut_greedy_impr(g))
        });
    }

    time_group.finish();
}

criterion_group!(time, approx_bench, heuristic_bench, heuristic_approx_bench);
criterion_main!(time);
