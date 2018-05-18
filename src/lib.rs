//! Parser for AppStream metadata.

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate license_exprs;
extern crate regex;
#[macro_use]
extern crate serde;
extern crate serde_xml_rs;
extern crate sxd_document;
extern crate tld as tld_rs;
extern crate url;
#[macro_use]
extern crate url_serde;
extern crate xpath_reader;

pub mod comp_type;
pub mod field;
pub mod metainfo;

use xpath_reader::Reader;

use self::field::copyright::Copyright;
use self::field::icon::Icon;
use self::field::id::Id;
use self::field::license::License;
use self::field::name::Name;
use self::field::pkg_name::PkgName;
use self::field::summary::Summary;
use self::field::Field;
use self::metainfo::ParseError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppStream {
    copyright: Copyright,
    id: Id,
    name: Name,
    summary: Summary,
    pkg_name: PkgName,
    license: Option<License>,
    metadata_license: Option<License>,
    icons: Option<Vec<Icon>>,
}

impl AppStream {
    pub(crate) fn parse<'d>(reader: Reader<'d>) -> Result<Self, ParseError> {
        Ok(AppStream {
            copyright: parse_field(&reader)?,
            id: parse_field(&reader)?,
            name: parse_field(&reader)?,
            summary: parse_field(&reader)?,
            pkg_name: parse_field(&reader)?,
            license: parse_field(&reader)?,
            metadata_license: parse_field(&reader)?,
            icons: None,
        })
    }

    pub fn copyright(&self) -> &Copyright {
        &self.copyright
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn summary(&self) -> &Summary {
        &self.summary
    }

    pub fn pkg_name(&self) -> &PkgName {
        &self.pkg_name
    }

    pub fn license(&self) -> Option<&License> {
        self.license.as_ref()
    }

    pub fn metadata_license(&self) -> Option<&License> {
        self.metadata_license.as_ref()
    }

    pub fn icons(&self) -> Option<&[Icon]> {
        self.icons.as_ref().map(|vec| vec.as_slice())
    }
}

fn parse_field<'d, F: Field>(reader: &Reader<'d>) -> Result<F, ParseError> {
    let input = F::load(&reader).map_err(ParseError::Xpath)?;
    F::construct(input)
        .map_err(|e| e.into())
        .map_err(ParseError::FieldParseFail)
}
