// @generated
// This file is @generated by prost-build.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EscalatedPrivilege {
    All = 0,
    ReloadPlugin = 1,
    LoadPlugin = 2,
    UnloadPlugin = 3,
    InstructPlugin = 4,
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
            EscalatedPrivilege::LoadPlugin => "LOAD_PLUGIN",
            EscalatedPrivilege::UnloadPlugin => "UNLOAD_PLUGIN",
            EscalatedPrivilege::InstructPlugin => "INSTRUCT_PLUGIN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ALL" => Some(Self::All),
            "RELOAD_PLUGIN" => Some(Self::ReloadPlugin),
            "LOAD_PLUGIN" => Some(Self::LoadPlugin),
            "UNLOAD_PLUGIN" => Some(Self::UnloadPlugin),
            "INSTRUCT_PLUGIN" => Some(Self::InstructPlugin),
            _ => None,
        }
    }
}
/// Arguments for a plugin or a plugins subcommands.
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(default, setter(into, strip_option), custom_constructor, build_fn(name = "fallible_build")))]
#[serde(default)]
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
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(default, setter(into, strip_option), custom_constructor, build_fn(name = "fallible_build")))]
#[serde(default)]
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
    /// Access checks
    ///
    /// This command can optionally be gated by access checks.
    #[prost(message, optional, tag="8")]
    pub access_checks: ::core::option::Option<super::access_check::AccessCheckChain>,
}
/// Metadata for a plugin.
#[cfg_attr(feature = "kameo", derive(kameo::Reply))]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(default, setter(into, strip_option), custom_constructor, build_fn(name = "fallible_build")))]
#[serde(default)]
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
    /// Hosts that this plugin requests permissions to access via HTTP.
    #[prost(string, repeated, tag="15")]
    pub allowed_hosts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Pool size this plugin requests.
    #[prost(int32, optional, tag="16")]
    pub pool_size: ::core::option::Option<i32>,
}
/// An argument passed to a commands context.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandContextArgument {
    /// The name of the argument.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The value of the argument.
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
/// Plugin command context.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandContext {
    /// The name of the command.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Arguments passed to the command.
    #[prost(message, repeated, tag="2")]
    pub arguments: ::prost::alloc::vec::Vec<CommandContextArgument>,
    /// Optional nested subcommand context.
    #[prost(message, optional, boxed, tag="3")]
    pub subcommand: ::core::option::Option<::prost::alloc::boxed::Box<CommandContext>>,
}
/// \[Internal\] Kinds of instructions that can be sent to hank plugins.
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InstructionKind {
    /// Plugin entry point.
    Plugin = 0,
    /// Get the plugins metadata.
    GetMetadata = 1,
    /// Call the plugins install function. (only happens once)
    Install = 2,
    /// Call the plugins initialize function. (happens on every load)
    Initialize = 3,
    /// Call the plugins shutdown function. (happens on reload, unload, uninstall)
    Shutdown = 4,
    /// Call the plguins chat message handler.
    ChatMessage = 5,
    /// Call the plguins chat command handler.
    ChatCommand = 6,
    /// Call the plguins scheduled job handler.
    ScheduledJob = 7,
}
impl InstructionKind {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InstructionKind::Plugin => "Plugin",
            InstructionKind::GetMetadata => "GetMetadata",
            InstructionKind::Install => "Install",
            InstructionKind::Initialize => "Initialize",
            InstructionKind::Shutdown => "Shutdown",
            InstructionKind::ChatMessage => "ChatMessage",
            InstructionKind::ChatCommand => "ChatCommand",
            InstructionKind::ScheduledJob => "ScheduledJob",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Plugin" => Some(Self::Plugin),
            "GetMetadata" => Some(Self::GetMetadata),
            "Install" => Some(Self::Install),
            "Initialize" => Some(Self::Initialize),
            "Shutdown" => Some(Self::Shutdown),
            "ChatMessage" => Some(Self::ChatMessage),
            "ChatCommand" => Some(Self::ChatCommand),
            "ScheduledJob" => Some(Self::ScheduledJob),
            _ => None,
        }
    }
}
/// \[Internal\] An instruction to send to hank plugin.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instruction {
    /// The kind of instruction to send to the hank plugin.
    #[prost(enumeration="InstructionKind", tag="1")]
    pub kind: i32,
    /// An input to send to the hank plugin.
    #[prost(bytes="vec", tag="2")]
    pub input: ::prost::alloc::vec::Vec<u8>,
    /// An optional target plugin name to send the instruciton to.
    #[prost(string, optional, tag="3")]
    pub target: ::core::option::Option<::prost::alloc::string::String>,
}
// @@protoc_insertion_point(module)
// Customizations from hank.plugin.customizations.rs
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
