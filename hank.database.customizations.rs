// Customizations from hank.database.customizations.rs
#[cfg(feature = "builder")]
impl PreparedStatement {
    pub fn new(sql: impl Into<String>) -> PreparedStatementBuilder {
        PreparedStatementBuilder {
            sql: Some(sql.into()),
            ..PreparedStatementBuilder::create_empty()
        }
    }
}
#[cfg(feature = "builder")]
impl PreparedStatementBuilder {
    pub fn values(&mut self, value: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
        self.values = Some(value.into_iter().map(Into::into).collect());
        self
    }
    pub fn build(&self) -> PreparedStatement {
        self.fallible_build()
            .expect("All required fields were initialized")
    }
}
