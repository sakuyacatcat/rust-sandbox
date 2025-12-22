use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn dies_no_args() {
    cargo_bin_cmd!("echor")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

fn runs(args: &[&str], expected: &str) {
    cargo_bin_cmd!("echor")
        .args(args)
        .assert()
        .success()
        .stdout(predicate::eq(expected));
}


#[test]
fn hello1() {
    runs(&["Hello, world!"], "Hello, world!\n")
}

#[test]
fn hello2() {
    runs(&["Hello,", "world!"], "Hello, world!\n")
}

#[test]
fn hello1_no_newline() {
    runs(&["-n", "Hello, world!"], "Hello, world!")
}

#[test]
fn hello2_no_newline() {
    runs(&["-n", "Hello,", "world!"], "Hello, world!")
}
