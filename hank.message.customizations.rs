// Customizations from hank.database.customizations.rs
#[cfg(feature = "builder")]
impl Reaction {
    pub fn new(emoji: impl Into<String>, message: Message) -> ReactionBuilder {
        ReactionBuilder {
            emoji: Some(emoji.into()),
            message: Some(Some(message)),
        }
    }
}
#[cfg(feature = "builder")]
impl ReactionBuilder {
    pub fn build(&self) -> Reaction {
        self.fallible_build()
            .expect("All required fields were initialized")
    }
}
