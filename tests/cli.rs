use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::error::Error;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("sp")?;
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

    let mut cmd = Command::cargo_bin("sp")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("1:A test\n4:A\x00nother test"));

    Ok(())
}

#[test]
fn find_content_in_file_no_line_number() -> Result<(), Box<dyn Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "A test\nActual content\r\nMore content\nA\x00nother test"
    )?;
    let mut cmd = Command::cargo_bin("sp")?;
    cmd.arg("Test").arg(file.path()).arg("-n").arg("-i");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nA\x00nother test"));

    Ok(())
}

#[test]
fn find_content_in_file_line_number() -> Result<(), Box<dyn Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "A test\nActual content\r\nMore content\nA\x00nother test"
    )?;
    let mut cmd = Command::cargo_bin("sp")?;
    cmd.arg("test").arg(file.path()).arg("-m=1");
    cmd.assert()
        .success()
        .stdout(predicate::str::similar("1:A test\n"));

    Ok(())
}

#[test]
fn empty_string_search() -> Result<(), Box<dyn Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "A test\nActual content\nMore content\nA\x00nother test"
    )?;

    let mut cmd = Command::cargo_bin("sp")?;
    cmd.arg("").arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}

// TODO: Add test count without patter argument
#[test]
fn count_match() -> Result<(), Box<dyn Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "A test\nActual content\nMore content\nA\x00nother test"
    )?;

    let mut cmd = Command::cargo_bin("sp")?;
    cmd.arg("A").arg(file.path()).arg("-c").arg("-i");
    cmd.assert().success().stdout(predicate::str::contains("3"));

    Ok(())
}
