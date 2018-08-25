use std::iter::Iterator;
use std::path::PathBuf;

use failure::Error;
use url::Url;

use super::Field;

#[derive(Debug)]
pub struct Icons<'a> {
    icons: &'a [Icon],
    cur: usize,
}

impl<'a> Icons<'a> {
    pub(crate) fn new(icons: &'a [Icon]) -> Self {
        Icons {
            icons,
            cur: 0,
        }
    }
}

impl<'a> Iterator for Icons<'a> {
    type Item = &'a Icon;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur += 1;
        self.icons.get(self.cur)
    }
}

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
    type Input = Vec<String>;
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
