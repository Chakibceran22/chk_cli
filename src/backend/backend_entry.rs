use crate::{backend::nest::nest_generate::nest_generate, types::app_options::BackendOptions, utils::utils::{check_package_manager_installed, get_node_package_manager_option, should_use_git}};
use cliclack::{outro_note, select};
pub fn backend_run(project_name : &str) -> anyhow::Result<()>{
    let backend_choice = select("Choose your starter template.")
    .item(BackendOptions::Nest, "NestJs", "Nest Starter template")
    .item(BackendOptions::JavaSpringBoot, "Spring Boot", "Template starters for Spring Boot")
    .interact()?;
    

    match backend_choice {
        BackendOptions::Nest => {
            let package_manager = get_node_package_manager_option()?;
            check_package_manager_installed(package_manager)?;
            let git_choice = should_use_git()?;
             nest_generate(project_name, git_choice, package_manager)?;
            outro_note(
                "Next steps",
                format!("cd {project_name}\n{} run start:dev", package_manager.as_str()),
            )?;





        }
        BackendOptions::JavaSpringBoot => {

        }
    }

    Ok(())
}