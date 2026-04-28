use assert_cmd::prelude::*;
use std::process::Command;
use predicates::prelude::*;

#[test]
fn it_generates_a_password() {
    let cmd = Command::cargo_bin("password-generator").unwrap();
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("[a-zA-Z0-9]"))
        .stdout(predicates::str::length(range(8, 20)));
}

#[test]
fn it_respects_length_parameter() {
    let cmd = Command::cargo_bin("password-generator")
        .unwrap()
        .arg("--length")
        .arg("8");
    cmd.assert()
        .success()
        .stdout(predicates::str::length(8));
}

#[test]
fn it_respects_count_parameter() {
    let cmd = Command::cargo_bin("password-generator")
        .unwrap()
        .arg("--count")
        .arg("3");
    cmd.assert()
        .success()
        .stdout(predicates::str::lines(3));
}

#[test]
fn it_excludes_uppercase() {
    let cmd = Command::cargo_bin("password-generator")
        .unwrap()
        .arg("--lower")
        .arg("--digits");
    cmd.assert()
        .success()
        .stdout(predicates::str::not(any_of('A'..='Z')));
}

#[test]
fn it_fails_when_no_character_types_selected() {
    let cmd = Command::cargo_bin("password-generator")
        .unwrap()
        .arg("--upper")
        .arg("--lower")
        .arg("--digits")
        .arg("--symbols")
        .arg("--exclude")
        .arg('a')
        .arg("--exclude")
        .arg('A')
        .arg("--exclude")
        .arg('0')
        .arg("--exclude")
        .arg('!');
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("No character types selected"));
}

#[test]
fn test_basic_generation() {
    let mut cmd = Command::cargo_bin("password-generator").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn it_respects_lower_option() {
    let mut cmd = Command::cargo_bin("password-generator").unwrap();
    cmd.arg("--lower")
        .arg("false")
        .assert()
        .failure();
}

#[test]
fn it_respects_upper_option() {
    let mut cmd = Command::cargo_bin("password-generator").unwrap();
    cmd.arg("--upper")
        .arg("false")
        .assert()
        .failure();
}

#[test]
fn test_length_option() {
    let mut cmd = Command::cargo_bin("password-generator").unwrap();
    cmd.arg("--length")
        .arg("15")
        .assert()
        .success()
        .stdout(predicate::str::len(15));
}

#[test]
fn test_count_option() {
    let mut cmd = Command::cargo_bin("password-generator").unwrap();
    cmd.arg("--count")
        .arg("3")
        .assert()
        .success();
}

#[test]
fn test_exclusion_functionality() {
    let mut cmd = Command::cargo_bin("password-generator").unwrap();
    cmd.arg("--exclude")
        .arg("a")
        .assert()
        .success();
}

