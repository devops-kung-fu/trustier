
use anyhow::{Ok, Result};
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn dies_no_args() -> Result<()> {
    let mut cmd = Command::cargo_bin("trustier")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}