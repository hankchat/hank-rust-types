// @generated
// This file is @generated by prost-build.
/// Metadata for a plugin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// The plguins name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// A short description of the plugin.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// A version string for the plugin. Should follow semver.
    ///
    /// @see: <https://semver.org/>
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
    /// When true, a SQLite3 database will be created for the plugin.
    #[prost(bool, tag="4")]
    pub database: bool,
}
// @@protoc_insertion_point(module)
