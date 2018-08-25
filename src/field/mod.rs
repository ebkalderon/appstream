use std::fmt::Debug;

use failure::Fail;
use xpath_reader::{Error as XpathError, FromXml, Reader};

pub mod category;
pub mod copyright;
pub mod icon;
pub mod id;
pub mod license;
pub mod name;
pub mod pkg_name;
pub mod summary;

pub trait Field: Sized + Debug {
    type Input: FromXml;
    type Error: Fail + Send + Sync + 'static;

    const XPATH_EXPR: &'static str;

    fn construct(input: Self::Input) -> Result<Self, Self::Error>;

    fn load<'d>(reader: &'d Reader<'d>) -> Result<Self::Input, XpathError> {
        reader.read(Self::XPATH_EXPR)
    }
}
