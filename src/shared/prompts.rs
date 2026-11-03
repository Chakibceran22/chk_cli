
use cliclack::{confirm, input, select};

use crate::types::app_options::Appoptions;

pub fn prompt_app_choices() -> anyhow::Result<Appoptions> {
     let app_choice = select("Choose your app option.")
    .item(Appoptions::Backend, "Backend", "Templates starters for backend projects")
    .item(Appoptions::Frontend, "Frontend", "Templates starters for frontend projects")
    .item(Appoptions::Mobile, "Mobile", "Template starters for Mobile apps")
    .item(Appoptions::Desktop, "Desktop", "Template starters for desktop applications")
    .interact()?;
    Ok(app_choice)
}


pub fn prompt_project_name() -> anyhow::Result<String>{
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



pub fn prompt_should_use_git() -> anyhow::Result<bool>{

    let git_choice  = confirm("Do You wanna Initialize git.")
    .interact()?;
    Ok(git_choice)
}




