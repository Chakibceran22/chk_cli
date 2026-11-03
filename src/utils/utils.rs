use crate::types::app_options::{NodePackageManagerOptions};
use std::process::{Command, Stdio};
use anyhow::bail;


pub fn check_node_package_manager_installed(package_manager: NodePackageManagerOptions) -> anyhow::Result<()> {
    let pm = package_manager.as_str();
    // stdin closed so a corepack "download yarn?" question can't hang us
    let installed = Command::new(pm)
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success());

    if !installed {
        bail!("{pm} isn't installed. Pick another package manager or install it first.");
    }
    Ok(())
}
pub fn is_cancelled(err: &anyhow::Error) -> bool {
    err.downcast_ref::<std::io::Error>()
        .is_some_and(|e| e.kind() == std::io::ErrorKind::Interrupted)
}