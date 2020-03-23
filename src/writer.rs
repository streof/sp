use bstr::BString;
use std::io::Write;
use std::str;

pub fn print_matches(
    mut writer: impl Write,
    matches: Result<Vec<BString>, std::io::Error>,
) -> std::result::Result<(), anyhow::Error> {
    for single_match in matches.unwrap().iter() {
        writeln!(
            writer,
            "{}",
            str::from_utf8(single_match.as_slice())
                .expect("Found invalid UTF-8")
                .trim_end()
        )?;
    }
    Ok(())
}
