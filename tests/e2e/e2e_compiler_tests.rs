// tests/e2e_compiler_tests.rs

use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::error::Error;

#[test]
fn test_compile_and_run_simple_program_native() -> Result<(), Box<dyn Error>> {
    let temp = assert_fs::TempDir::new()?;
    let input_file = temp.child("program.md");
    input_file.write_str(
        r#":::naldom
Create an array of 5 random numbers.
Sort it in ascending order.
Print the result.
:::"#,
    )?;
    let output_executable = temp.child("my_program");

    let mut cmd = Command::cargo_bin("naldom-cli")?;
    cmd.arg(input_file.path())
        .arg("-o")
        .arg(output_executable.path());
    cmd.assert().success();

    let mut run_cmd = Command::new(output_executable.path());
    run_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("--- Naldom Native Output ---"));

    Ok(())
}
