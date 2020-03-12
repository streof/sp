use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, BufReader};
use structopt::StructOpt;

fn main() -> Result<(), anyhow::Error> {
    // Parse arguments
    let args = grrs::Cli::from_args();

    // Read file into buffer
    let f =
        File::open(&args.path).with_context(|| format!("Could not read file {:?}", &args.path))?;
    let reader = BufReader::new(f);

    // Get a locked stdout wrapped in a buffer
    let stdout = io::stdout();
    let handle = io::BufWriter::new(stdout.lock());

    // Read and match lines
    grrs::find_matches(reader, &args, handle)
}
