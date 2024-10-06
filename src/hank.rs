// @generated
// This file is @generated by prost-build.
/// \[Internal\] Input to a send message request to Hank.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMessageInput {
    /// The message to send to Hank.
    #[prost(message, optional, tag="1")]
    pub message: ::core::option::Option<message::Message>,
}
/// \[Internal\] Output from a send message request to Hank.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SendMessageOutput {
}
/// \[Internal\] Input to a reaction request to Hank.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactInput {
    /// The reaction to send to Hank.
    #[prost(message, optional, tag="1")]
    pub reaction: ::core::option::Option<message::Reaction>,
}
/// \[Internal\] Output from a reaction request to Hank.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ReactOutput {
}
/// \[Internal\] Input to a db query request to Hank.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DbQueryInput {
    /// The prepared statement to send to Hank.
    #[prost(message, optional, tag="1")]
    pub prepared_statement: ::core::option::Option<database::PreparedStatement>,
}
/// \[Internal\] Output from a db query request to Hank.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DbQueryOutput {
    /// The database results from the query from Hank.
    #[prost(message, optional, tag="1")]
    pub results: ::core::option::Option<database::Results>,
}
/// \[Internal\] Input to a cron request to Hank.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CronInput {
    /// A cronjob to send to Hank.
    #[prost(message, optional, tag="1")]
    pub cron_job: ::core::option::Option<cron::CronJob>,
}
/// \[Internal\] Output from a cron request to Hank.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CronOutput {
}
/// \[Internal\] Input to a one shot request to Hank.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneShotInput {
    /// A one shot to send to Hank.
    #[prost(message, optional, tag="1")]
    pub one_shot_job: ::core::option::Option<cron::OneShotJob>,
}
/// \[Internal\] Output from a one shot request to Hank.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct OneShotOutput {
}
/// \[Internal\] Input to a reload plugin request to Hank.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadPluginInput {
    /// The plugin to reload.
    #[prost(string, tag="1")]
    pub plugin: ::prost::alloc::string::String,
}
/// \[Internal\] Output from a reload plugin request to Hank.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ReloadPluginOutput {
}
/// \[Internal\] Input to a load plugin request to Hank.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadPluginInput {
    /// The url to a compiled plugins wasm file to load.
    #[prost(string, tag="1")]
    pub url: ::prost::alloc::string::String,
}
/// \[Internal\] Output from a load plugin request to Hank.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadPluginOutput {
    /// The metadata returned by the loaded plugin.
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<plugin::Metadata>,
}
/// \[Internal\] Input to a handle chat command request to Hank.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandleChatCommandInput {
    /// The chat command context to send to Hank.
    #[prost(message, optional, tag="1")]
    pub context: ::core::option::Option<plugin::CommandContext>,
    /// The message that the chat command originates from.
    #[prost(message, optional, tag="2")]
    pub message: ::core::option::Option<message::Message>,
}
/// \[Internal\] Output from a handle chat command request to Hank.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct HandleChatCommandOutput {
}
// @@protoc_insertion_point(module)
