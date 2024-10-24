// Customizations from hank.plugin.customizations.rs
#[cfg(feature = "builder")]
#[derive(Debug)]
pub enum AccessChecks {
    Array(Vec<crate::access_check::AccessCheck>),
    Single(crate::access_check::AccessCheck),
    Full(crate::access_check::AccessCheckChain),
}
#[cfg(feature = "builder")]
impl Metadata {
    pub fn new(
        name: impl Into<String>,
        author: impl Into<String>,
        description: impl Into<String>,
        version: impl Into<String>,
    ) -> MetadataBuilder {
        MetadataBuilder {
            name: Some(name.into()),
            author: Some(author.into()),
            description: Some(description.into()),
            version: Some(version.into()),
            ..MetadataBuilder::create_empty()
        }
    }
}
#[cfg(feature = "builder")]
impl MetadataBuilder {
    pub fn allowed_hosts(
        &mut self,
        value: impl IntoIterator<Item = impl Into<String>>,
    ) -> &mut Self {
        self.allowed_hosts = Some(value.into_iter().map(Into::into).collect());
        self
    }
    pub fn aliases(&mut self, value: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
        self.aliases = Some(value.into_iter().map(Into::into).collect());
        self
    }
    pub fn escalation_key(&mut self, value: impl Into<String>) -> &mut Self {
        self.escalation_key = Some(Some(value.into()));
        self
    }
    pub fn escalated_privileges(
        &mut self,
        value: impl IntoIterator<Item = impl Into<i32>>,
    ) -> &mut Self {
        self.escalated_privileges = Some(value.into_iter().map(Into::into).collect());
        self
    }
    pub fn access_checks(&mut self, value: AccessChecks) -> &mut Self {
        self.access_checks = Some(match value {
            AccessChecks::Array(checks) => Some(crate::access_check::AccessCheckChain {
                operator: crate::access_check::AccessCheckOperator::Or.into(),
                checks,
            }),
            AccessChecks::Single(check) => Some(crate::access_check::AccessCheckChain {
                operator: crate::access_check::AccessCheckOperator::Or.into(),
                checks: vec![check],
            }),
            AccessChecks::Full(full) => Some(full),
        });
        self
    }
    pub fn build(&self) -> Metadata {
        self.fallible_build()
            .expect("All required fields were initialized")
    }
}
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
        self.fallible_build()
            .expect("All required fields were initialized")
    }
}
