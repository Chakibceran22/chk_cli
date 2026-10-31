use crate::{backend::nest::nest_generate::nest_generate, types::app_options::BackendOptions, utils::utils::{get_node_package_manager_option, should_use_git}};
use cliclack::{log, select};
pub fn backend_entry(project_name : &str) -> std::io::Result<()> {
    let backend_choice = select("Choose your starter template.")
    .item(BackendOptions::Nest, "NestJs", "Nest Starter template")
    .item(BackendOptions::JavaSpringBoot, "Spring Boot", "Template starters for Spring Boot")
    .interact()?;
    

    match backend_choice {
        BackendOptions::Nest => {
            let package_manager = get_node_package_manager_option()?;
            let git_choice = should_use_git()?;
            let _ = nest_generate(project_name, git_choice, package_manager);




        }
        BackendOptions::JavaSpringBoot => {

        }
    }

    Ok(())
}