#![allow(missing_docs, clippy::expect_used)]

use std::process::Command;

#[test]
fn fixture_manifest_verifies_offline() {
    let binary = env!("CARGO_BIN_EXE_xtask");
    let output = Command::new(binary)
        .args(["fixtures", "verify"])
        .output()
        .expect("xtask fixtures verify should run");

    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
