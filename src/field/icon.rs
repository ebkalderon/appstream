use std::path::PathBuf;

use failure::Error;
use url::Url;

use super::Field;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Icon {
    Stock {
        id: String,
    },
    Cached {
        name: String,
    },
    Local {
        path: PathBuf,
        width: Option<u32>,
        height: Option<u32>,
    },
    Remote {
        url: Url,
        width: Option<u32>,
        height: Option<u32>,
    },
}

impl Field for Option<Icon> {
    type Input = Vec<Option<String>>;
    type Error = ParseError;

    const XPATH_EXPR: &'static str = "/component/icon[@type]/text()";

    fn construct(input: Self::Input) -> Result<Self, Self::Error> {
        println!("{:?}", input);
        Ok(Some(Icon::Stock { id: "".to_string() }))
    }
}

#[derive(Clone, Debug, Fail)]
pub enum ParseError {
    #[fail(display = "Unexpected field `{}` with value `{}`", _0, _1)]
    UnexpctedAttribute { field: String, value: String },
    #[fail(display = "Invalid icon type `{}`", _0)]
    InvalidType(String),
}
