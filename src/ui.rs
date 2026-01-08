use crate::model::RunResult;

/* =======================
 * ANSI
 * ======================= */
macro_rules! fg { ($c:expr) => { format!("\x1b[38;5;{}m", $c) }; }
macro_rules! bg { ($c:expr) => { format!("\x1b[48;5;{}m", $c) }; }

const RESET: &str = "\x1b[0m";
const BOLD: &str  = "\x1b[1m";
const DIM: &str   = "\x1b[2m";

const C_OK: u8    = 82;
const C_ERR: u8   = 196;
const C_WARN: u8  = 214;
const C_INFO: u8  = 81;
const C_CMD: u8   = 75;
const C_MUTED: u8 = 245;
const C_TIME: u8  = 141;

/* =======================
 * ICONS
 * ======================= */
const I_OK: &str   = "󰄬";
const I_ERR: &str  = "󰅙";
const I_INFO: &str = "󰋼";
const I_SH: &str   = "";
const I_TIME: &str = "󰅐";
const I_OUT: &str  = "󰆍";
const I_WARN: &str = "󰀪";

fn h(title: &str) {
    println!("\n{}{}{} {}{}", fg!(C_INFO), BOLD, I_INFO, title, RESET);
}

fn row(icon: &str, color: u8, label: &str, value: impl std::fmt::Display) {
    println!(
        "  {}{}{} {:<14} {}{}{}",
        fg!(color), icon, RESET,
        label,
        fg!(250), value, RESET
    );
}

fn badge(icon: &str, color: u8, text: &str) -> String {
    format!("{}{} {} {}  {}", bg!(color), fg!(16), icon, text, RESET)
}

pub fn render(result: &RunResult) {
    h("Command");
    println!("  {}{}{} {}", fg!(C_CMD), I_SH, RESET, result.command);

    h("Result");
    println!(
        "  {}",
        if result.success {
            badge(I_OK, C_OK, "SUCCESS")
        } else {
            badge(I_ERR, C_ERR, "FAILED")
        }
    );

    row(I_SH,   C_CMD,  "exit", result.exit);
    row(I_TIME, C_TIME, "time", format!("{} ms", result.duration_ms));

    h("Output");
    row(I_OUT, C_MUTED, "stdout", result.out_lines);
    row(I_OUT, C_MUTED, "stderr", result.err_lines);

    row(
        if result.warnings > 0 { I_WARN } else { I_OK },
        if result.warnings > 0 { C_WARN } else { C_OK },
        "warnings",
        result.warnings,
    );

    row(
        if result.errors > 0 { I_ERR } else { I_OK },
        if result.errors > 0 { C_ERR } else { C_OK },
        "errors",
        result.errors,
    );

    if !result.success && !result.stderr.is_empty() {
        h("stderr preview");
        for line in result.stderr.lines().take(6) {
            println!("  {}│{} {}", fg!(240), RESET, line);
        }
    }

    println!("\n{}{}done{}", fg!(244), DIM, RESET);
}
