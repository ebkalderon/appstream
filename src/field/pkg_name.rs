use std::fmt::{Display, Formatter, Result as FmtResult};

use super::Field;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PkgName(String);

impl Display for PkgName {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let PkgName(ref name) = *self;
        name.fmt(fmt)
    }
}

impl From<String> for PkgName {
    fn from(s: String) -> Self {
        PkgName(s)
    }
}

impl Field for PkgName {
    type Input = String;
    type Error = PkgNameLoadError;

    const XPATH_EXPR: &'static str = "/component/pkgname/text()";

    fn construct(input: Self::Input) -> Result<Self, Self::Error> {
        Ok(PkgName(input))
    }
}

#[derive(Clone, Debug, Fail)]
#[fail(display = "Failed to load `pkgname`")]
pub struct PkgNameLoadError;
