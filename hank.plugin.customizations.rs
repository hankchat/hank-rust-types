// Customizations from hank.plugin.customizations.rs
impl Command {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> CommandBuilder {
        CommandBuilder {
            name: Some(name.into()),
            description: Some(description.into()),
            ..CommandBuilder::create_empty()
        }
    }
}
impl CommandBuilder {
    pub fn build(&self) -> Command {
        self.fallible_build()
            .expect("All required fields were initialized")
    }
}
impl Argument {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> ArgumentBuilder {
        ArgumentBuilder {
            name: Some(name.into()),
            description: Some(description.into()),
            ..ArgumentBuilder::create_empty()
        }
    }
}
impl ArgumentBuilder {
    pub fn build(&self) -> Argument {
        self.fallible_build()
            .expect("All required fields were initialized")
    }
}
