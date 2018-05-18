use std::fmt::{Display, Formatter, Result as FmtResult};

use super::Field;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Summary(String);

impl Display for Summary {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let Summary(ref name) = *self;
        name.fmt(fmt)
    }
}

impl From<String> for Summary {
    fn from(s: String) -> Self {
        Summary(s)
    }
}

impl Field for Summary {
    type Input = String;
    type Error = SummaryLoadError;

    const XPATH_EXPR: &'static str = "/component/summary/text()";

    fn construct(input: Self::Input) -> Result<Self, Self::Error> {
        Ok(Summary(input))
    }
}

#[derive(Clone, Debug, Fail)]
#[fail(display = "Failed to load `summary`")]
pub struct SummaryLoadError;
