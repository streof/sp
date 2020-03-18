use crate::cli::Cli;
use bstr::{io::BufReadExt, ByteSlice};
use std::io::{BufRead, Write};
use std::str;

pub fn find_matches(
    reader: impl BufRead,
    cli: &Cli,
    mut writer: impl Write,
) -> std::result::Result<(), anyhow::Error> {
    reader.for_byte_line_with_terminator(|line| {
        if line.contains_str(&cli.pattern) {
            writeln!(
                writer,
                "{}",
                str::from_utf8(&line)
                    .expect("Found invalid UTF-8")
                    .trim_end()
            )?;
        }
        Ok(true)
    })?;

    // Return () on success
    Ok(())
}

#[test]
fn find_a_match() {
    use std::path::PathBuf;
    use std::str::FromStr;

    let file = "tests/data/sample.txt";
    let cli = Cli {
        path: PathBuf::from_str(file).unwrap(),
        pattern: "lorem".to_owned(),
    };

    let reader = &std::fs::read(file).unwrap()[..];
    let mut result = Vec::new();
    find_matches(reader, &cli, &mut result).unwrap();
    assert_eq!(result, b"lorem ipsum\n")
}

// TODO write a few more tests
