use assert_cmd::prelude::*;
use std::process::Command;

fn run_password(args: &[&str]) -> String {
    let output = Command::cargo_bin("password-generator")
        .unwrap()
        .args(args)
        .output()
        .unwrap();

    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn it_generates_a_password() {
    let password = run_password(&["--lower", "--digits"]);
    let password = password.trim();

    assert!(!password.is_empty());
    assert_eq!(password.len(), 12);
}

#[test]
fn it_respects_length_parameter() {
    let password = run_password(&["--lower", "--length", "8"]);
    assert_eq!(password.trim().len(), 8);
}

#[test]
fn it_respects_count_parameter() {
    let output = run_password(&["--lower", "--count", "3"]);
    assert_eq!(output.lines().count(), 3);
}

#[test]
fn it_excludes_uppercase() {
    let output = run_password(&["--lower", "--digits"]);
    assert!(!output.trim().chars().any(|c| c.is_ascii_uppercase()));
}

#[test]
fn it_fails_when_no_character_types_selected() {
    Command::cargo_bin("password-generator")
        .unwrap()
        .assert()
        .failure()
        .stderr(predicates::str::contains("No character types selected"));
}

#[test]
fn test_basic_generation() {
    let password = run_password(&["--symbols"]);
    assert!(!password.trim().is_empty());
}

#[test]
fn it_respects_lower_option() {
    let password = run_password(&["--lower", "--length", "6"]);
    assert!(password.trim().chars().all(|c| c.is_ascii_lowercase()));
}

#[test]
fn it_respects_upper_option() {
    let password = run_password(&["--upper", "--length", "6"]);
    assert!(password.trim().chars().all(|c| c.is_ascii_uppercase()));
}

#[test]
fn test_length_option() {
    let password = run_password(&["--digits", "--length", "15"]);
    assert_eq!(password.trim().len(), 15);
}

#[test]
fn test_count_option() {
    let output = run_password(&["--digits", "--count", "3"]);
    assert_eq!(output.lines().count(), 3);
}

#[test]
fn test_exclusion_functionality() {
    let password = run_password(&["--lower", "--exclude", "a", "--length", "20"]);
    assert!(!password.contains('a'));
}

#[test]
fn it_includes_each_selected_requirement() {
    let assert = Command::cargo_bin("password-generator")
        .unwrap()
        .arg("--length")
        .arg("4")
        .arg("--upper")
        .arg("--lower")
        .arg("--digits")
        .arg("--symbols")
        .assert()
        .success();

    let output = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let password = output.trim();

    assert!(password.chars().any(|c| c.is_ascii_uppercase()));
    assert!(password.chars().any(|c| c.is_ascii_lowercase()));
    assert!(password.chars().any(|c| c.is_ascii_digit()));
    assert!(password.chars().any(|c| !c.is_ascii_alphanumeric()));
}

#[test]
fn it_fails_when_length_is_shorter_than_selected_requirements() {
    Command::cargo_bin("password-generator")
        .unwrap()
        .arg("--length")
        .arg("2")
        .arg("--lower")
        .arg("--digits")
        .arg("--symbols")
        .assert()
        .failure()
        .stderr(predicates::str::contains("too short to satisfy 3 selected requirements"));
}

    #[test]
    fn it_fails_when_exclusions_make_a_selected_requirement_impossible() {
        Command::cargo_bin("password-generator")
        .unwrap()
        .arg("--lower")
        .arg("--exclude")
        .arg("a")
        .arg("--exclude")
        .arg("b")
        .arg("--exclude")
        .arg("c")
        .arg("--exclude")
        .arg("d")
        .arg("--exclude")
        .arg("e")
        .arg("--exclude")
        .arg("f")
        .arg("--exclude")
        .arg("g")
        .arg("--exclude")
        .arg("h")
        .arg("--exclude")
        .arg("i")
        .arg("--exclude")
        .arg("j")
        .arg("--exclude")
        .arg("k")
        .arg("--exclude")
        .arg("l")
        .arg("--exclude")
        .arg("m")
        .arg("--exclude")
        .arg("n")
        .arg("--exclude")
        .arg("o")
        .arg("--exclude")
        .arg("p")
        .arg("--exclude")
        .arg("q")
        .arg("--exclude")
        .arg("r")
        .arg("--exclude")
        .arg("s")
        .arg("--exclude")
        .arg("t")
        .arg("--exclude")
        .arg("u")
        .arg("--exclude")
        .arg("v")
        .arg("--exclude")
        .arg("w")
        .arg("--exclude")
        .arg("x")
        .arg("--exclude")
        .arg("y")
        .arg("--exclude")
        .arg("z")
        .assert()
        .failure()
        .stderr(predicates::str::contains("Cannot satisfy the selected lowercase requirement"));
    }

