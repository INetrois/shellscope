pub struct RunResult {
    pub command: String,
    pub exit: i32,
    pub duration_ms: u128,
    pub stdout: String,
    pub stderr: String,
    pub out_lines: usize,
    pub err_lines: usize,
    pub warnings: usize,
    pub errors: usize,
    pub success: bool,
}
