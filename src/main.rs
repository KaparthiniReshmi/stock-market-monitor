use std::process::ExitCode;

use clap::Parser;
use stock_monitor::Cli;

fn main() -> ExitCode {
    let cli = Cli::parse();
    match stock_monitor::run(&cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
