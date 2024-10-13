// @generated
#[cfg(feature = "hank")]
// @@protoc_insertion_point(attribute:hank)
pub mod hank {
    include!("hank.rs");
    // @@protoc_insertion_point(hank)
    #[cfg(feature = "hank-access_check")]
    // @@protoc_insertion_point(attribute:hank.access_check)
    pub mod access_check {
        include!("hank.access_check.rs");
        // @@protoc_insertion_point(hank.access_check)
    }
    #[cfg(feature = "hank-channel")]
    // @@protoc_insertion_point(attribute:hank.channel)
    pub mod channel {
        include!("hank.channel.rs");
        // @@protoc_insertion_point(hank.channel)
    }
    #[cfg(feature = "hank-cron")]
    // @@protoc_insertion_point(attribute:hank.cron)
    pub mod cron {
        include!("hank.cron.rs");
        // @@protoc_insertion_point(hank.cron)
    }
    #[cfg(feature = "hank-database")]
    // @@protoc_insertion_point(attribute:hank.database)
    pub mod database {
        include!("hank.database.rs");
        // @@protoc_insertion_point(hank.database)
    }
    #[cfg(feature = "hank-message")]
    // @@protoc_insertion_point(attribute:hank.message)
    pub mod message {
        include!("hank.message.rs");
        // @@protoc_insertion_point(hank.message)
    }
    #[cfg(feature = "hank-plugin")]
    // @@protoc_insertion_point(attribute:hank.plugin)
    pub mod plugin {
        include!("hank.plugin.rs");
        // @@protoc_insertion_point(hank.plugin)
    }
    #[cfg(feature = "hank-user")]
    // @@protoc_insertion_point(attribute:hank.user)
    pub mod user {
        include!("hank.user.rs");
        // @@protoc_insertion_point(hank.user)
    }
}
// Customizations from lib.customizations.rs
pub use hank::*;

impl serde::ser::Serialize for access_check::AccessCheckChain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeStruct as _;
        let mut state = serializer.serialize_struct("AccessCheckChain", 1)?;
        state.serialize_field(
            access_check::AccessCheckOperator::try_from(self.operator)
                .expect("invalid access check operator")
                .as_str_name(),
            &self.checks,
        )?;

        state.end()
    }
}

impl<'de> serde::de::Deserialize<'de> for access_check::AccessCheckChain {
    fn deserialize<D: serde::de::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let d: std::collections::BTreeMap<String, Vec<access_check::AccessCheck>> =
            std::collections::BTreeMap::deserialize(d)?;

        let (operator, checks) = d.first_key_value().expect("invalid access chain format");
        Ok(access_check::AccessCheckChain {
            operator: access_check::AccessCheckOperator::from_str_name(operator)
                .expect("invalid access check operator")
                .into(),
            checks: checks.to_vec(),
        })
    }
}

#[derive(Default)]
pub struct InstructionBuilder<T>
where
    T: Default + prost::Message,
{
    kind: plugin::InstructionKind,
    input: T,
    target: Option<String>,
}

impl<T> InstructionBuilder<T>
where
    T: Default + prost::Message,
{
    pub fn new(kind: plugin::InstructionKind) -> Self {
        Self {
            kind,
            ..Default::default()
        }
    }

    pub fn with_input(self, input: T) -> Self {
        Self { input, ..self }
    }

    pub fn with_target(self, target: String) -> Self {
        Self {
            target: Some(target),
            ..self
        }
    }

    pub fn build(self) -> plugin::Instruction {
        self.into()
    }
}

impl<T> From<InstructionBuilder<T>> for plugin::Instruction
where
    T: Default + prost::Message,
{
    fn from(value: InstructionBuilder<T>) -> Self {
        plugin::Instruction {
            kind: value.kind.into(),
            input: value.input.encode_to_vec(),
            target: value.target,
        }
    }
}
