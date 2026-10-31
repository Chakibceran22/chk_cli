use anyhow::bail;
use cliclack::{confirm, input, select};
use std::process::{Command, Stdio};

use crate::types::app_options::NodePackageManagerOptions;

pub fn get_project_name() -> anyhow::Result<String>{
    let project_name : String = input("Enter your project name")
    .placeholder("my-project")
    .validate(|input: &String| {
        if input.trim().is_empty() {
            Err("Project name can't be empty")
        }else {
            Ok(())
        }
    })
    .interact()?;
    Ok(project_name.trim().to_string())
}

pub fn get_node_package_manager_option() -> anyhow::Result<NodePackageManagerOptions> {
    let package_manager_choice = select("Choose your package manager")
    .item(NodePackageManagerOptions::Npm, "npm","")
    .item(NodePackageManagerOptions::Pnpm, "pnpm","")
    .item(NodePackageManagerOptions::Yarn, "yarn","")
    .interact()?;
    Ok(package_manager_choice)
}

pub fn check_package_manager_installed(package_manager: NodePackageManagerOptions) -> anyhow::Result<()> {
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

pub fn should_use_git() -> anyhow::Result<bool>{

    let git_choice  = confirm("Do You wanna Initialize git.")
    .interact()?;
    Ok(git_choice)
}

pub fn is_cancelled(err: &anyhow::Error) -> bool {
    err.downcast_ref::<std::io::Error>()
        .is_some_and(|e| e.kind() == std::io::ErrorKind::Interrupted)
}