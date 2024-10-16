// @generated
// This file is @generated by prost-build.
/// Database Query Results.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Results {
    /// An array of JSON strings.
    #[prost(string, repeated, tag="1")]
    pub rows: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A prepared statement to execute on the database.
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(default, setter(into, strip_option), custom_constructor, build_fn(name = "fallible_build")))]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreparedStatement {
    /// The SQL query to execute, using ? for placeholders.
    ///
    /// Example:
    ///   SELECT * FROM table WHERE id = ?
    #[prost(string, tag="1")]
    pub sql: ::prost::alloc::string::String,
    /// An array of values to substitute each ? placeholder in the SQL query with.
    #[prost(string, repeated, tag="2")]
    #[cfg_attr(feature = "builder", builder(setter(custom)))]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
// @@protoc_insertion_point(module)
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
