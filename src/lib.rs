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
extern crate psl;
extern crate url;
extern crate xpath_reader;

pub mod comp_type;
pub mod component;
pub mod field;
pub mod metainfo;

use xpath_reader::Reader;

use field::category::Categories;
use field::copyright::Copyright;
use field::icon::Icon;
use field::id::Id;
use field::license::License;
use field::name::Name;
use field::pkg_name::PkgName;
use field::summary::Summary;
use field::Field;
use metainfo::ParseError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppStream {
    copyright: Copyright,
    id: Id,
    pkg_name: PkgName,
    name: Name,
    summary: Summary,
    license: Option<License>,
    metadata_license: Option<License>,
    icons: Option<Vec<Icon>>,
    categories: Option<Categories>,
}

impl AppStream {
    pub(crate) fn parse<'d>(reader: Reader<'d>) -> Result<Self, ParseError> {
        Ok(AppStream {
            copyright: parse_field(&reader)?,
            id: parse_field(&reader)?,
            pkg_name: parse_field(&reader)?,
            name: parse_field(&reader)?,
            summary: parse_field(&reader)?,
            license: parse_field(&reader)?,
            metadata_license: parse_field(&reader)?,
            icons: None,
            categories: parse_field(&reader)?,
        })
    }

    pub fn copyright(&self) -> &Copyright {
        &self.copyright
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn pkg_name(&self) -> &PkgName {
        &self.pkg_name
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn summary(&self) -> &Summary {
        &self.summary
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

    pub fn categories(&self) -> Option<&Categories> {
        self.categories.as_ref()
    }
}

fn parse_field<'d, F: Field>(reader: &Reader<'d>) -> Result<F, ParseError> {
    let input = F::load(&reader).map_err(ParseError::Xpath)?;
    F::construct(input)
        .map_err(|e| e.into())
        .map_err(ParseError::FieldParseFail)
}
