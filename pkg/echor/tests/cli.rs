use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

fn runs(args: &[&str], expected: &str) -> TestResult {
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(predicate::eq(expected));
    Ok(())
}


#[test]
fn hello1() -> TestResult {
    runs(&["Hello, world!"], "Hello, world!\n")
}

#[test]
fn hello2() -> TestResult {
    runs(&["Hello,", "world!"], "Hello, world!\n")
}

#[test]
fn hello1_no_newline() -> TestResult {
    runs(&["-n", "Hello, world!"], "Hello, world!")
}

#[test]
fn hello2_no_newline() -> TestResult {
    runs(&["-n", "Hello,", "world!"], "Hello, world!")
}
