use std::fmt::{Display, Formatter, Result as FmtResult};

use super::Field;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Name(String);

impl Display for Name {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let Name(ref name) = *self;
        name.fmt(fmt)
    }
}

impl From<String> for Name {
    fn from(s: String) -> Self {
        Name(s)
    }
}

impl Field for Name {
    type Input = String;
    type Error = NameLoadError;

    const XPATH_EXPR: &'static str = "/component/name/text()";

    fn construct(input: Self::Input) -> Result<Self, Self::Error> {
        Ok(Name(input))
    }
}

#[derive(Clone, Debug, Fail)]
#[fail(display = "Failed to load `name`")]
pub struct NameLoadError;
