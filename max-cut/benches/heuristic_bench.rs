use std::{fs, time::Duration};

use criterion::{
    criterion_group, criterion_main,
    measurement::{Measurement, ValueFormatter},
    BenchmarkId, Criterion, PlotConfiguration, Throughput,
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
            "graph: {} vetices: {}, edges: {}",
            graph_path.file_name().unwrap().to_str().unwrap(),
            graph.size(),
            graph.edge_size()
        );

        time_group.throughput(criterion::Throughput::Elements(graph.size() as u64));
        time_group.sample_size(10);
        time_group.measurement_time(Duration::from_secs(10));
        // time_group.warm_up_time(Duration::from_millis(1));
        time_group.plot_config(PlotConfiguration::default());
        time_group.bench_with_input(BenchmarkId::new("approx basic: ", &id), &graph, |b, g| {
            b.iter(|| aprox::max_cut_greedy(g))
        });
        time_group.bench_with_input(BenchmarkId::new("approx improved: ", &id), &graph, |b, g| {
            b.iter(|| aprox::max_cut_greedy_impr(g))
        });
    }

    time_group.finish();


}

fn approx_size_bench(c: &mut Criterion<ResultSize>) {
    let mut result_group = c.benchmark_group("result_group");

    let graphs = fs::read_dir("benches/data/rudy").unwrap();

    for graph_path in graphs {
        let graph_path = graph_path.unwrap().path();
        let raw = fs::read_to_string(&graph_path).unwrap();
        let graph = graph_parser::parse_rudy(&raw).unwrap();

        let id = format!(
            "graph: {} vetices: {}, edges: {}",
            graph_path.file_name().unwrap().to_str().unwrap(),
            graph.size(),
            graph.edge_size()
        );

        result_group.sample_size(1000);
        result_group.measurement_time(Duration::from_secs(10));
        result_group.throughput(Throughput::Elements((graph.size() + graph.edge_size()) as u64));
        // result_group.warm_up_time(Duration::from_millis(1));
        result_group.sampling_mode(criterion::SamplingMode::Flat);
        result_group.bench_with_input(BenchmarkId::new("approx basic: ", &id), &graph, |b, g| {
            b.iter_custom(|iters| {
                let mut size = 0;
                for _i in 0..iters {
                    size += aprox::max_cut_greedy(g).len();
                }

                size
            })
        });
        result_group.bench_with_input(BenchmarkId::new("approx improved: ", &id), &graph, |b, g| {
            b.iter_custom(|iters| {
                let mut size = 0;
                for _i in 0..iters {
                    size += aprox::max_cut_greedy_impr(g).len();
                }
                size
            })
        });
    }

    result_group.finish();
}

struct ResultSize;
impl Measurement for ResultSize {
    type Intermediate = usize;
    type Value = usize;

    fn start(&self) -> Self::Intermediate {
        0usize
    }

    fn end(&self, i: Self::Intermediate) -> Self::Value {
        i
    }

    fn add(&self, v1: &Self::Value, v2: &Self::Value) -> Self::Value {
        *v1 + *v2
    }

    fn zero(&self) -> Self::Value {
        0usize
    }

    fn to_f64(&self, value: &Self::Value) -> f64 {
        let i = *value as f64;
        i
    }

    fn formatter(&self) -> &dyn criterion::measurement::ValueFormatter {
        &UsizeFormatter
    }
}

struct UsizeFormatter;
impl ValueFormatter for UsizeFormatter {
    fn format_value(&self, value: f64) -> String {
        format!("{} edges", value)
    }
    fn format_throughput(&self, throughput: &criterion::Throughput, value: f64) -> String {
        format!("{}", value)
    }
    fn scale_values(&self, typical_value: f64, values: &mut [f64]) -> &'static str {
        "edges"
    }
    fn scale_throughputs(
        &self,
        typical_value: f64,
        throughput: &Throughput,
        values: &mut [f64],
    ) -> &'static str {
        ""
    }
    fn scale_for_machines(&self, values: &mut [f64]) -> &'static str {
        "edges"
    }
}

criterion_group!(time, approx_bench);
// criterion_group!{
//     name = size;
//     config = Criterion::default().with_measurement(ResultSize);
//     targets = approx_size_bench
// }
criterion_main!(time);
