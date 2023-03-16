#!/bin/bash

echo "graph, vertices, edges, cut-size, time in ms" > ilp_bench_result.csv


for graph in data/vc_exact/* ; do
  cargo run --release -- --ilp 10 -b $graph >> ilp_bench_result.csv
  # echo $graph
done
