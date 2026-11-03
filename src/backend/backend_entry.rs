use crate::{backend::nest::nest_generate::nest_generate, shared::prompts::{ prompt_should_use_git}, types::app_options::BackendOptions, utils::utils::check_node_package_manager_installed};
use cliclack::{outro_note};
use crate::backend::prompts::{prompt_backend_choices, prompt_node_package_manager_option};
use crate::backend::nest::nest_additional_packages::nest_additional_packages;

pub fn backend_run(project_name : &str) -> anyhow::Result<()>{
    let backend_choice = prompt_backend_choices()?;
    

    match backend_choice {
        BackendOptions::Nest => {
            let package_manager = prompt_node_package_manager_option()?;
            check_node_package_manager_installed(package_manager);
            let git_choice = prompt_should_use_git()?;
            let additional_packages = nest_additional_packages()?;

            nest_generate(project_name, git_choice, package_manager, additional_packages)?;
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