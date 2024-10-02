// @generated
// This file is @generated by prost-build.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EscalatedPrivilege {
    All = 0,
    ReloadPlugin = 1,
}
impl EscalatedPrivilege {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EscalatedPrivilege::All => "ALL",
            EscalatedPrivilege::ReloadPlugin => "RELOAD_PLUGIN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ALL" => Some(Self::All),
            "RELOAD_PLUGIN" => Some(Self::ReloadPlugin),
            _ => None,
        }
    }
}
/// Arguments for a plugin or a plugins subcommands.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Argument {
    /// Argument name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Argument description.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Optional argument default value.
    #[prost(string, optional, tag="3")]
    pub default_value: ::core::option::Option<::prost::alloc::string::String>,
    /// Whether or not this argument is required.
    #[prost(bool, tag="4")]
    pub required: bool,
    /// Sets the short version of the argument without the preceding -.
    #[prost(string, optional, tag="5")]
    pub short: ::core::option::Option<::prost::alloc::string::String>,
    /// Sets the long version of the argument without the preceding --.
    #[prost(string, optional, tag="6")]
    pub long: ::core::option::Option<::prost::alloc::string::String>,
}
/// Plugin commands.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Command {
    /// Command name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Command description.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Command author.
    #[prost(string, optional, tag="3")]
    pub author: ::core::option::Option<::prost::alloc::string::String>,
    /// A version string for the command. Should follow semver.
    ///
    /// @see: <https://semver.org/>
    #[prost(string, optional, tag="4")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    /// Command aliases.
    #[prost(string, repeated, tag="5")]
    pub aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Command arguments.
    #[prost(message, repeated, tag="6")]
    pub arguments: ::prost::alloc::vec::Vec<Argument>,
    /// Command subcommands.
    #[prost(message, repeated, tag="7")]
    pub subcommands: ::prost::alloc::vec::Vec<Command>,
}
/// Metadata for a plugin.
#[cfg_attr(feature = "kameo", derive(kameo::Reply))]
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
    /// @deprecated All plugins get a database by default now.
    #[prost(bool, tag="4")]
    pub database: bool,
    /// Access checks
    ///
    /// All functionality of this plugin can optionally be gated by accses checks.
    #[prost(message, optional, tag="5")]
    pub access_checks: ::core::option::Option<super::access_check::AccessCheckChain>,
    /// A secret escalation key that grants this plugin specific escalated
    /// privileges.
    #[prost(string, optional, tag="6")]
    pub escalation_key: ::core::option::Option<::prost::alloc::string::String>,
    /// A list of escalated privileges that this plugin requests to use.
    #[prost(enumeration="EscalatedPrivilege", repeated, tag="7")]
    pub escalated_privileges: ::prost::alloc::vec::Vec<i32>,
    /// The author of the plugin.
    #[prost(string, tag="8")]
    pub author: ::prost::alloc::string::String,
    /// Whether or not this plugin handles commands.
    #[prost(bool, tag="9")]
    pub handles_commands: bool,
    /// Whether or not this plugin handles messages.
    #[prost(bool, tag="10")]
    pub handles_messages: bool,
    /// Optionally override the plugin command name.
    #[prost(string, optional, tag="11")]
    pub command_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional aliases for the plugin command.
    #[prost(string, repeated, tag="12")]
    pub aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Arguments for the plugin command.
    #[prost(message, repeated, tag="13")]
    pub arguments: ::prost::alloc::vec::Vec<Argument>,
    /// Plugin subcommands.
    #[prost(message, repeated, tag="14")]
    pub subcommands: ::prost::alloc::vec::Vec<Command>,
}
// @@protoc_insertion_point(module)
