use std::fs;

use assert_cmd::Command;

const BINARY_NAME: &str = "num2_echor";
const EXPECTED_DIRECTORY: &str = "tests/expected/";

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;
    cmd.arg("Hello").assert().success();
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "hello2.n.txt")
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let outfile = format!("{}{}", EXPECTED_DIRECTORY, expected_file);
    let expected = fs::read_to_string(outfile)?;
    Command::cargo_bin(BINARY_NAME)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
