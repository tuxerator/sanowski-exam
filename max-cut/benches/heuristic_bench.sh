#!/bin/bash

echo "graph, vertices, edges, cut-size, time in ms" > heuristic_bench_result.csv


for graph in data/vc_exact/*; do
  cargo run --release -- -bi --heuristic $graph >> heuristic_bench_result.csv
  # echo $graph
done
