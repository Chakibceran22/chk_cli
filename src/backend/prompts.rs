use crate::types::app_options::{BackendOptions, NodePackageManagerOptions};
use cliclack::select;

pub fn prompt_backend_choices() -> anyhow::Result<BackendOptions>{
    let backend_choice = select("Choose your starter template.")
    .item(BackendOptions::Nest, "NestJs", "Nest Starter template")
    .item(BackendOptions::JavaSpringBoot, "Spring Boot", "Template starters for Spring Boot")
    .interact()?;
    Ok(backend_choice)

}

pub fn prompt_node_package_manager_option() -> anyhow::Result<NodePackageManagerOptions> {
    let package_manager_choice = select("Choose your package manager")
    .item(NodePackageManagerOptions::Npm, "npm","")
    .item(NodePackageManagerOptions::Pnpm, "pnpm","")
    .item(NodePackageManagerOptions::Yarn, "yarn","")
    .interact()?;
    Ok(package_manager_choice)
}


