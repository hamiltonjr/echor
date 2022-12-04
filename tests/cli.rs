use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

/*
 * This is a bunch of tests for the echor project.
 */

type TestResult = Result<(), Box<dyn std::error::Error>>;

// this function is used for verify some test examples
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// this test verify if the code runs properly
#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

// this test verify if the code prints properly with 1 arg with \n
#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

// this test verify if the code prints properly with 2 args with \n
#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

// this test verify if the code prints properly with 1 arg and no \n
#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello there", "-n"], "tests/expected/hello1.n.txt")
}

// this test verify if the code prints properly with 2 args and no \n
#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
