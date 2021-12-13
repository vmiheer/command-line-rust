use assert_cmd::Command;
use predicates::prelude::*;
use std;

type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert().failure().stderr(predicate::str::contains("USAGE"));
}

#[test]
fn runs_with_newline() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello").arg("World").assert().success().stdout("Hello World\n");
}

#[test]
fn runs_without_newline() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello").arg("World").arg("-n").assert().success().stdout("Hello World");
}

#[test]
fn hello_1() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    let expected = std::fs::read_to_string("tests/expected/hello1.txt")?;
    cmd.arg("Hello  there").assert().success().stdout(expected);
    return Ok(());
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(args).assert().success().stdout(std::fs::read_to_string(expected_file)?);
    Ok(())
}

#[test]
fn test_using_fn() -> TestResult {
    run(&["Hello  there"], "tests/expected/hello1.txt")
}