use bstr::BString;
use std::io::Write;
use std::str;
pub fn print_matches(
    mut writer: impl Write,
    matches: Result<Vec<BString>, std::io::Error>,
) -> Result<(), anyhow::Error> {
    match matches {
        Ok(single_match) => print_lines_iter(&mut writer, &single_match).expect("Error occured"),
        Err(_) => println!("Error occured"),
    };
    Ok(())
}

pub fn print_lines_iter(writer: &mut impl Write, lines: &[BString]) -> Result<(), anyhow::Error> {
    for line in lines.iter() {
        writeln!(
            writer,
            "{}",
            str::from_utf8(line.as_slice())
                .expect("Found invalid UTF-8")
                .trim_end()
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use bstr::BString;
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom, Write};

    const DICKENS: &str = "\
He started      \r
made a run
& stopped\
";

    #[test]
    fn text_output() {
        let expected = "\
He started
made a run
& stopped
";
        // Convert to correct format
        let mut matches = vec![];
        DICKENS.lines().for_each(|x| matches.push(BString::from(x)));

        // Write to temp file
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        print_lines_iter(Write::by_ref(&mut tmpfile), &matches).unwrap();

        // Seek to start (!)
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        // Read back
        let mut got = String::new();
        tmpfile.read_to_string(&mut got).unwrap();

        assert_eq!(expected, got);
    }
}
