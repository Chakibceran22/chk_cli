
use cliclack::{intro};

use crate::shared::prompts::{prompt_app_choices, prompt_project_name};
use crate::types::app_options::Appoptions;
use crate::backend::backend_entry::backend_run;

pub fn main_runner()  -> anyhow::Result<()>{
    intro("Welcome to the Chk CLI")?;
    let app_choice = prompt_app_choices()?;
    let project_name: String = prompt_project_name()?;

    match app_choice {
        Appoptions::Backend => {
             backend_run(&project_name)?;

        }
        Appoptions::Frontend => {

        }
        Appoptions::Mobile => {

        }
        Appoptions::Desktop => {

        }
        
    }






    Ok(())
}