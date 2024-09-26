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
