//! SPDX license expression parsing.

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use license_exprs::{validate_license_expr, ParseError as LicenseParseError};
use serde::de::{Deserialize, Deserializer, Error as DeError, Visitor};
use serde::ser::{Serialize, Serializer};

use field::Field;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct License(String);

impl License {
    pub fn new<E: Into<String>>(expr: E) -> Result<Self, ParseError> {
        let inner = expr.into();
        validate_license_expr(&inner).map_err(ParseError::from)?;
        Ok(License(inner))
    }
}

impl<'de> Deserialize<'de> for License {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LicenseVisitor;

        impl<'de> Visitor<'de> for LicenseVisitor {
            type Value = License;

            fn expecting(&self, fmt: &mut Formatter) -> FmtResult {
                fmt.write_str("string containing a valid SPDX license expression")
            }

            fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
                License::new(v).map_err(DeError::custom)
            }
        }

        deserializer.deserialize_str(LicenseVisitor)
    }
}

impl Display for License {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let License(ref text) = *self;
        text.fmt(fmt)
    }
}

impl FromStr for License {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        License::new(s)
    }
}

impl Serialize for License {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let out = self.to_string();
        serializer.serialize_str(&out)
    }
}

impl Field for Option<License> {
    type Input = Option<String>;
    type Error = ParseError;

    const XPATH_EXPR: &'static str = "/component/license/text()";

    fn construct(input: Self::Input) -> Result<Self, Self::Error> {
        match input {
            Some(s) => License::from_str(&s).map(Option::Some),
            None => Ok(None),
        }
    }
}

#[derive(Clone, Debug, Fail)]
pub enum ParseError {
    #[fail(display = "unknown license or other term: {}", _0)]
    UnknownLicenseId(String),
    #[fail(display = "invalid license expression")]
    InvalidStructure,
}

impl<'a> From<LicenseParseError<'a>> for ParseError {
    fn from(error: LicenseParseError<'a>) -> Self {
        match error {
            LicenseParseError::UnknownLicenseId(id) => ParseError::UnknownLicenseId(id.into()),
            LicenseParseError::InvalidStructure(_) => ParseError::InvalidStructure,
        }
    }
}
