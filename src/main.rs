


use std::process::ExitCode;

use crate::{entrypoint::cli, utils::utils::is_cancelled};
mod entrypoint;
mod types;
mod backend;
mod utils;

pub fn main() -> ExitCode {
    match cli() {
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