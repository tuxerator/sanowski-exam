# Max-Cut

## Build

To build the program install `cargo` and `clang` then execute
```
cargo build --release
```

The binaries can be found in **traget/**.


## Execute

Move into the **max-cut** directory and execute:
```
cargo run --release -- [args]
```

For more detailed information about the usage execute:
```
cargo run --release -- -h
```

## Test

### Run time

To benchmark the running time of all algorithms except the ILP run:
```
cargo bench
```
The results can be found in **target/criterion/**.
For easy viewing just open the **index.html** inside **report/**.

### Result size and ILP

For benchmarking the results scripts are provided in **bencher/**.
The results will be written to a .csv file inside the calling directory.
