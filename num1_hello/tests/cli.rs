use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("num1_hello").unwrap();
    cmd.assert().success();
}
