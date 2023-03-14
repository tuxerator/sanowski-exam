#!/bin/bash

echo "graph, vertices, edges, cut-size, time in ms" > approx_bench_result.csv


for graph in data/rudy/*; do
  cargo run --release -- -ba $graph >> approx_bench_result.csv
  # echo $graph
done
