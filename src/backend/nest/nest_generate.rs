use anyhow::{ bail};
use cliclack::spinner;
use crate::{ backend::nest::main_template::MainTemplate, types::app_options::{NestAdditionalPackages::{self, ClassValidatorAndransformer, Helmet}, NodePackageManagerOptions::{self, Npm, Pnpm, Yarn}}};
use std::process::Command;
pub fn nest_generate(project_name: &str, git_choice: bool, package_manager: NodePackageManagerOptions, additional_packages: Vec<NestAdditionalPackages>) -> anyhow::Result<()> {
    let validation = additional_packages.contains(&ClassValidatorAndransformer);
    let helmet = additional_packages.contains(&Helmet);
    let template = MainTemplate::new(validation, helmet);
    let main_ts = template.to_string();
    let sp = spinner();
    sp.start(format!("Scaffolding NestJs project '{project_name}'"));

    let (command, dlx_flag) = match  package_manager {
        Pnpm => ("pnpm", "dlx"),
        Npm | Yarn => ("npx", "--yes")
        
    };
    let mut args = vec![
        dlx_flag,
          "@nestjs/cli@12",
        "new",
        project_name,
        "--package-manager",
        package_manager.as_str(),

    ];

    if !git_choice {
        args.push("--skip-git");
    }

    let output = match Command::new(command)
    .args(&args)
    .output() {
        Ok(output) => output,
        Err(err) => {
             sp.error(format!("Couldn't start {command}"));
            bail!("Couldn't run `{command}`. Is it installed? ({err})");
        }
    };

     if !output.status.success() {
        sp.error("Nest CLI failed");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Nest CLI failed:\n{stdout}\n{stderr}");
    }
    std::fs::write(format!("{project_name}/src/main.ts"), main_ts);
    sp.stop(format!("NestJS project '{project_name}' initialized!"));
    
    Ok(())


    
}