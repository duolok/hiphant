use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn run_with_defaults() {
    Command::cargo_bin("hiphant")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("Memory overload"));
}

#[test]
fn run_with_custom_message() {
    Command::cargo_bin("hiphant")
        .expect("binary exists")
        .arg("Hello, world!")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, world!"));
}

#[test]
fn run_with_happy_eyes() {
    Command::cargo_bin("hiphant")
        .expect("binary exists")
        .args(&["-s", "Happy Elephant"])
        .assert()
        .success()
        .stdout(predicate::str::contains("^").and(predicate::str::contains("Happy Elephant")));
}

#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("hiphant")
        .expect("binary exists")
        .args(&["-f", "no/such/file.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Could not read file"));

    Ok(())
}
