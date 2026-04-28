use assert_cmd::prelude::*;
use std::process::Command;

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