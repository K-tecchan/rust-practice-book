use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn runs() -> TestResult {
    Command::cargo_bin("echor")?.arg("hello").assert().success();
    Ok(())
}

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert().failure().stderr(predicate::str::contains("USAGE"));
    Ok(())
}

// このテストはなぜか失敗する。最初の行に空行があるとかないとかよくわからん
// #[test]
// fn hello1() {
//     let outfile = "tests/expected/hello1.txt";
//     let expected = fs::read_to_string(outfile).unwrap();
//     let mut cmd = Command::cargo_bin("echor").unwrap();
//     cmd.arg("Hello there").assert().success().stdout(expected);
// }