use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use crate::model::RunResult;

pub fn write(result: &RunResult) {
    let mut dir = dirs::home_dir().expect("no home dir");
    dir.push(".shellscope");
    fs::create_dir_all(&dir).expect("failed to create ~/.shellscope");

    let ts = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let mut path = PathBuf::from(dir);
    path.push(format!("{}.log", ts));

    let mut file = File::create(path).expect("failed to create log");

    writeln!(file, "command: {}", result.command).unwrap();
    writeln!(file, "exit: {}", result.exit).unwrap();
    writeln!(file, "time_ms: {}", result.duration_ms).unwrap();
    writeln!(file, "stdout_lines: {}", result.out_lines).unwrap();
    writeln!(file, "stderr_lines: {}", result.err_lines).unwrap();
    writeln!(file, "warnings: {}", result.warnings).unwrap();
    writeln!(file, "errors: {}", result.errors).unwrap();

    writeln!(file, "\n--- stdout ---\n{}", result.stdout).unwrap();
    writeln!(file, "\n--- stderr ---\n{}", result.stderr).unwrap();
}
