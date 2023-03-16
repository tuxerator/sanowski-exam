use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input graph
    pub file: std::path::PathBuf,

    /// Calculate the maximum cut using an integer with timeout.
    /// If set to 0 no timeout is used.
    #[arg(long, default_value_t = -1)]
    pub ilp: i64,

    /// Calculate the maximum cut using the approximation algorithm
    #[arg(short, long)]
    pub approx: bool,

    /// Calculate the maximum cut using the heuristic
    #[arg(short = 'H', long)]
    pub heuristic: bool,

    /// Calculate the maximum cut using the paralell heuristic
    #[arg(long)]
    pub heuristic_parallel: bool,

    /// Benchmark the calculation
    #[arg(short, long)]
    pub bench: bool,

    /// Use the improved variant of the algorithm.
    /// Can't be used together with '--ipl'
    #[arg(short, long)]
    pub improved: bool,
}
