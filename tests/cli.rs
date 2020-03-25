use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::error::Error;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("foobar").arg("test/file/doesnt/exists");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "A test\nActual content\r\nMore content\nA\x00nother test"
    )?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test\nA\x00nother test"));

    Ok(())
}

#[test]
fn empty_string_search() -> Result<(), Box<dyn Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "A test\nActual content\nMore content\nA\x00nother test"
    )?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("").arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}
