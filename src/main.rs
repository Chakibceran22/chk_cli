


use crate::entrypoint::cli;
mod entrypoint;
mod types;
mod backend;
mod utils;

pub fn main() -> std::io::Result<()> {
    let _ = cli();
    Ok(())
}