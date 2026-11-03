#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appoptions {
    Backend,
    Frontend,
    Mobile,
    Desktop
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub enum BackendOptions {
    Nest,
    JavaSpringBoot
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub enum NodePackageManagerOptions {
    Npm,
    Pnpm,
    Yarn
}
impl NodePackageManagerOptions {
    pub fn as_str(&self) -> &'static str{
        match self {
            NodePackageManagerOptions::Pnpm => "pnpm",
            NodePackageManagerOptions::Npm => "npm",
            NodePackageManagerOptions::Yarn => "yarn",
        }

    }
    
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NestAdditionalPackages {
    Drizzle,
    ClassValidatorAndransformer,
    Helmet
}