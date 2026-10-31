
use cliclack::{intro, select};

use crate::types::app_options::Appoptions;
use crate::backend::backend_entry::backend_run;
use crate::utils::utils::get_project_name;
pub fn cli()  -> anyhow::Result<()>{
    intro("Welcome to the Chk CLI")?;
    let app_choice = select("Choose your app option.")
    .item(Appoptions::Backend, "Backend", "Templates starters for backend projects")
    .item(Appoptions::Frontend, "Frontend", "Templates starters for frontend projects")
    .item(Appoptions::Mobile, "Mobile", "Template starters for Mobile apps")
    .item(Appoptions::Desktop, "Desktop", "Template starters for desktop applications")
    .interact()?;
    let project_name: String = get_project_name()?;

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