//! Represents an AppStream package ID.

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use super::Field;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Id {
    tld: TopLevelDomain,
    vendor: String,
    product: String,
}

impl Id {
    pub fn new<S>(tld: TopLevelDomain, vendor: S, product: S) -> Id
    where
        S: Into<String>,
    {
        Id {
            tld,
            vendor: vendor.into(),
            product: product.into(),
        }
    }

    pub fn parse<S>(tld: S, vendor: S, product: S) -> Result<Id, ParseError>
    where
        S: AsRef<str>,
    {
        let fields = [tld.as_ref(), vendor.as_ref(), product.as_ref()];

        for (i, field) in fields.iter().enumerate() {
            for ch in field.chars() {
                if !ch.is_alphanumeric() && ch != '-' && ch != '_' && ch != '.' {
                    return Err(ParseError::InvalidCharacter { field: i, ch });
                }
            }
        }

        let tld = TopLevelDomain::from_str(&fields[0]).map_err(ParseError::UnrecognizedTld)?;
        let vendor = fields[1];
        let product = fields[2];

        Ok(Id::new(tld, vendor, product))
    }
}

impl Display for Id {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let out = format!("{}.{}.{}", self.tld, self.vendor, self.product);
        fmt.write_str(&out)
    }
}

impl FromStr for Id {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.splitn(3, '.').collect();

        if fields.len() != 3 {
            return Err(ParseError::WrongNumFields);
        }

        Id::parse(fields[0], fields[1], fields[2])
    }
}

impl Field for Id {
    type Input = String;
    type Error = ParseError;

    const XPATH_EXPR: &'static str = "/component/id/text()";

    fn construct(input: Self::Input) -> Result<Self, Self::Error> {
        Id::from_str(&input)
    }
}

#[derive(Clone, Debug, Fail)]
pub enum ParseError {
    #[fail(display = "Expected three fields separated by `.` characters")]
    WrongNumFields,
    #[fail(display = "Invalid character `{}` in field {}", ch, field)]
    InvalidCharacter { ch: char, field: usize },
    #[fail(display = "{}", _0)]
    UnrecognizedTld(#[cause] UnrecognizedTldError),
}

/// An IANA registered top-level domain.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TopLevelDomain(String);

impl Display for TopLevelDomain {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let TopLevelDomain(ref s) = *self;
        s.fmt(fmt)
    }
}

impl FromStr for TopLevelDomain {
    type Err = UnrecognizedTldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use psl::{List, Psl};

        let domain = s.to_string();
        if List::new().domain(&format!("example.{}", domain)).is_some() {
            Ok(TopLevelDomain(domain))
        } else {
            Err(UnrecognizedTldError(domain))
        }
    }
}

#[derive(Clone, Debug, Fail)]
#[fail(display = "Invalid IANA top-level domain: {}", _0)]
pub struct UnrecognizedTldError(String);
