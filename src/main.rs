mod model;
mod runner;
mod ui;
mod log;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: shellscope <command> [args...]");
        std::process::exit(1);
    }

    let result = runner::run(&args);

    log::write(&result);
    ui::render(&result);
}
