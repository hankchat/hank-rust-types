// @generated
// This file is @generated by prost-build.
/// A user role
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Role {
    /// The role id
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The role name
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
/// A chat user
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    /// The users id
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The users username
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The users display name
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
    /// Any roles the user might have
    #[prost(message, repeated, tag="4")]
    pub roles: ::prost::alloc::vec::Vec<Role>,
}
// @@protoc_insertion_point(module)
