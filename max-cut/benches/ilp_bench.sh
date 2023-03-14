#!/bin/bash

echo "graph, vertices, edges, cut-size, time in ms" > ilp_bench_result.csv

files=("data/rudy/g05_60.0" "data/rudy/g05_80.0" "data/rudy/g05_100.0")

for graph in ${files[@]}; do
  cargo run --release -- -bi $graph >> ilp_bench_result.csv
  # echo $graph
done
