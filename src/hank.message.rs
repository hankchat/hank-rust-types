// @generated
// This file is @generated by prost-build.
/// A chat message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    /// The id of a received message
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// The time the message was sent
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// The user who authored the message
    #[prost(message, optional, tag="3")]
    pub author: ::core::option::Option<super::user::User>,
    /// The channel the message is to/from
    #[prost(message, optional, tag="4")]
    pub channel: ::core::option::Option<super::channel::Channel>,
    /// The message content
    #[prost(string, tag="5")]
    pub content: ::prost::alloc::string::String,
}
/// A reaction to a message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reaction {
    /// A message to react to.
    #[prost(message, optional, tag="1")]
    pub message: ::core::option::Option<Message>,
    /// A utf-8 emoji to react with.
    #[prost(string, tag="2")]
    pub emoji: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
