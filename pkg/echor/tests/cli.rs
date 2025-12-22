use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    cargo_bin_cmd!("echor")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

fn runs(args: &[&str], expected: &str) -> TestResult {
    cargo_bin_cmd!("echor")
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
