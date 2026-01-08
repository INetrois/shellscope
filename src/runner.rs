use std::process::{Command, Stdio};
use std::time::Instant;
use std::io::Read;

use crate::model::RunResult;

pub fn run(args: &[String]) -> RunResult {
    let start = Instant::now();

    let mut child = Command::new(&args[0])
        .args(&args[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn");

    let mut stdout = String::new();
    let mut stderr = String::new();

    child.stdout.take().unwrap().read_to_string(&mut stdout).unwrap();
    child.stderr.take().unwrap().read_to_string(&mut stderr).unwrap();

    let status = child.wait().unwrap();
    let duration = start.elapsed().as_millis();

    let out_lines = stdout.lines().count();
    let err_lines = stderr.lines().count();

    let warnings = stderr.matches("warning").count();
    let errors   = stderr.matches("error").count();

    RunResult {
        command: args.join(" "),
        exit: status.code().unwrap_or(-1),
        duration_ms: duration,
        stdout,
        stderr,
        out_lines,
        err_lines,
        warnings,
        errors,
        success: status.success(),
    }
}
