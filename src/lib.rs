use std::io::{BufRead, Write};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(name = "PATTERN", help = "A pattern used for matching a sub-slice")]
    pub pattern: String,

    #[structopt(name = "PATH", parse(from_os_str), help = "A file to search")]
    pub path: PathBuf,

    #[structopt(short, long, help = "Prints any warning or error messages")]
    verbose: bool,
}

pub fn find_matches(
    mut reader: impl BufRead,
    cli: &Cli,
    mut writer: impl Write,
) -> std::result::Result<(), anyhow::Error> {
    let mut buf = String::new();
    loop {
        match reader.read_line(&mut buf) {
            // Stream has reached EOF when 0 bytes are read
            Ok(0) => {
                break;
            }
            // Actual pattern matching
            Ok(_) => {
                if buf.contains(&cli.pattern) {
                    writeln!(writer, "{}", buf.trim_end())?;
                }
                buf.clear();
            }
            // Print extra info such as invalid UTF-8 lines
            Err(e) => {
                if cli.verbose {
                    writeln!(writer, "warn: {}", e)?;
                }
                buf.clear();
                continue;
            }
        }
    }

    // Return () on success
    Ok(())
}

#[test]
fn find_a_match() {
    use std::str::FromStr;

    let file = "tests/data/sample.txt";
    let cli = Cli {
        path: PathBuf::from_str(file).unwrap(),
        pattern: "lorem".to_owned(),
        verbose: false,
    };

    let reader = &std::fs::read(file).unwrap()[..];
    let mut result = Vec::new();
    find_matches(reader, &cli, &mut result).unwrap();
    assert_eq!(result, b"lorem ipsum\n")
}

// TODO write a few more tests
