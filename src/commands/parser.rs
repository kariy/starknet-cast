use clap::{builder::TypedValueParser, PossibleValue};
use starknet::core::types::FieldElement;

#[derive(Debug, Clone, Copy)]
pub struct FieldElementParser;

#[allow(unused_variables)]
impl TypedValueParser for FieldElementParser {
    type Value = FieldElement;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let value = value.to_str().ok_or(clap::Error::raw(
            clap::ErrorKind::InvalidUtf8,
            "Invalid utf-8",
        ))?;

        if value.starts_with("0x") {
            FieldElement::from_hex_be(value)
                .map_err(|e| clap::Error::raw(clap::ErrorKind::InvalidValue, e.to_string()))
        } else {
            FieldElement::from_dec_str(value)
                .map_err(|e| clap::Error::raw(clap::ErrorKind::InvalidValue, e.to_string()))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenChoice {
    Ether,
    Dai,
    Other(FieldElement),
}

#[derive(Debug, Clone, Copy)]
pub struct TokenValueParser;

#[allow(unused_variables)]
impl TypedValueParser for TokenValueParser {
    type Value = TokenChoice;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let value = value.to_str().ok_or(clap::Error::raw(
            clap::ErrorKind::InvalidUtf8,
            "Invalid utf-8",
        ))?;

        let value = value.to_lowercase();
        match value.as_str() {
            "ether" => Ok(TokenChoice::Ether),

            "dai" => Ok(TokenChoice::Dai),

            _ => Ok(TokenChoice::Other(
                FieldElement::from_hex_be(&value)
                    .map_err(|e| clap::Error::raw(clap::ErrorKind::InvalidValue, e.to_string()))?,
            )),
        }
    }

    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = clap::PossibleValue<'static>> + '_>> {
        let possible_values: Vec<PossibleValue<'static>> =
            vec![PossibleValue::new("ether"), PossibleValue::new("dai")];
        Some(Box::new(possible_values.into_iter()))
    }
}