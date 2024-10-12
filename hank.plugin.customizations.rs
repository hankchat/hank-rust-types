// Customizations from hank.plugin.customizations.rs
#[cfg(feature = "builder")]
impl Command {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> CommandBuilder {
        CommandBuilder {
            name: Some(name.into()),
            description: Some(description.into()),
            ..CommandBuilder::create_empty()
        }
    }
}
#[cfg(feature = "builder")]
impl CommandBuilder {
    pub fn build(&self) -> Command {
        self.fallible_build()
            .expect("All required fields were initialized")
    }
}
#[cfg(feature = "builder")]
impl Argument {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> ArgumentBuilder {
        ArgumentBuilder {
            name: Some(name.into()),
            description: Some(description.into()),
            ..ArgumentBuilder::create_empty()
        }
    }
}
#[cfg(feature = "builder")]
impl ArgumentBuilder {
    pub fn build(&self) -> Argument {
        self.fallible_build().map_err(|e| println!("{:?}", e));
    }
}
