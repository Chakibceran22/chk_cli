use cliclack::spinner;

use crate::types::app_options::NodePackageManagerOptions;
use std::process::Command;

pub fn nest_generate(project_name: &str, git_choice: bool, package_manager: NodePackageManagerOptions) -> std::io::Result<()> {
    let mut sp = spinner();
    sp.start(format!("Scaffolding NestJs projet '{project_name}'"));

    let (command, dlx_flag) = match  package_manager.as_str() {
        "pnpm" => ("pnpm", "dlx"),
        "yarn" => ("yarn", "dlx"),
        _ => ("npx", "--yes")
        
    };
    let mut args = Vec::new();
    if !dlx_flag.is_empty() {
        args.push(dlx_flag);
    }
    args.extend([
        "@nestjs/cli",
        "new",
        project_name,
        "--package-manager",
        package_manager.as_str()
    ]);

    if !git_choice {
        args.push("--skip-git");
    }

    let output = Command::new(command)
    .args(&args)
    .output()?;

    if !output.status.success() {
       sp.stop("Failed to scaffold NestJS project.");
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Nest CLI failed",
                ));
    }
    sp.stop(format!("NestJS project '{project_name}' initialized!"));

    Ok(())


    
}