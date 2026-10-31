use anyhow::{ bail};
use cliclack::spinner;

use crate::types::app_options::NodePackageManagerOptions::{self, Npm, Pnpm, Yarn};
use std::process::Command;

pub fn nest_generate(project_name: &str, git_choice: bool, package_manager: NodePackageManagerOptions) -> anyhow::Result<()> {
    let sp = spinner();
    sp.start(format!("Scaffolding NestJs project '{project_name}'"));

    let (command, dlx_flag) = match  package_manager {
        Pnpm => ("pnpm", "dlx"),
        // Yarn 1 has no `dlx`; Nest still installs with yarn thanks to --package-manager
        Npm | Yarn => ("npx", "--yes")
        
    };
    let mut args = vec![
        dlx_flag,
          "@nestjs/cli",
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
    sp.stop(format!("NestJS project '{project_name}' initialized!"));
    Ok(())


    
}