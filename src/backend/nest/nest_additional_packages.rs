use cliclack::multiselect;

use crate::types::app_options::NestAdditionalPackages::{self, ClassValidatorAndransformer, Drizzle, Helmet};
pub fn nest_additional_packages() -> anyhow::Result<Vec<NestAdditionalPackages>>{
    let user_packages = multiselect("Choose your additional packages to use")
    .item(Drizzle, "drizzle", "Base drizzle template with a clean architecture config")
    .item(ClassValidatorAndransformer, "class-validator + class-transformer", "Basic Validation Pipe for class validators and Dtos")
    .item(Helmet, "helmet", "Basic securty template for the package ")
    .interact()?;
    Ok(user_packages)
}