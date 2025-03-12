// tests/integration_tests.rs

use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_cli_help() {
    // This test assumes your binary is named "depclean"
    let mut cmd = Command::cargo_bin("depclean").expect("Binary 'depclean' exists");
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(contains("A tool to analyze and optimize project dependencies"));
}
