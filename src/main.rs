use anyhow::Context;
use grrs::cli::{Input, Output};
use std::fs::File;
use std::io::{self, BufReader};
use structopt::StructOpt;

fn main() -> Output {
    // Parse arguments
    let args = Input::from_args();

    // Read file into buffer
    let f =
        File::open(&args.path).with_context(|| format!("Could not read file {:?}", &args.path))?;
    let reader = BufReader::new(f);

    // Get a locked stdout wrapped in a buffer
    let stdout = io::stdout();
    let handle = io::BufWriter::new(stdout.lock());

    // Read and match lines
    args.show_matches(reader, handle)
}
