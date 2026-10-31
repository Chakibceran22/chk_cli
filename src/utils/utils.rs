use cliclack::{confirm, input, select};

use crate::types::app_options::NodePackageManagerOptions;

pub fn get_project_name() -> std::io::Result<String> {
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
    Ok(project_name)
}

pub fn get_node_package_manager_option() -> std::io::Result<NodePackageManagerOptions> {
    let package_manager_choice = select("Choose your package manager")
    .item(NodePackageManagerOptions::Npm, "npm","")
    .item(NodePackageManagerOptions::Pnpm, "pnpm","")
    .item(NodePackageManagerOptions::Yarn, "yarn","")
    .interact()?;
    Ok(package_manager_choice)
}

pub fn should_use_git() -> std::io::Result<bool> {
    let git_choice  = confirm("Do You wanna Initialize git.")
    .interact()?;
    Ok(git_choice)
}