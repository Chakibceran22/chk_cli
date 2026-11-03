


use std::process::ExitCode;
mod cli;
use crate::{cli::main_runner, utils::utils::is_cancelled};
mod types;
mod backend;
mod utils;
mod shared;

pub fn main() -> ExitCode {
    match main_runner() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) if is_cancelled(&err) => {
            let _ = cliclack::outro_cancel("Cancelled");
            ExitCode::from(130)
        }
        Err(err) => {
            let _ = cliclack::log::error(format!("{err:#}"));
            let _ = cliclack::outro_cancel("Scaffolding failed");
            ExitCode::FAILURE

        }

    }
}