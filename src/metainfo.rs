use failure::Error;
use sxd_document::parser::Error as XmlError;
use xpath_reader::{Error as XpathError, Reader};

use comp_type::{ComponentType, InvalidComponentType};
use AppStream;

pub struct Metainfo<'d>(Result<Reader<'d>, ParseError>);

impl<'d> Metainfo<'d> {
    pub fn from_str<S: AsRef<str> + 'd>(xml: S) -> Self {
        let trimmed = xml.as_ref().trim();
        let read = Reader::from_str(trimmed, None).map_err(ParseError::Xpath);
        Metainfo(read)
    }

    pub fn validate(self) -> Result<AppStream, ParseError> {
        let Metainfo(inner) = self;
        let reader = inner?;
        AppStream::parse(reader)
    }
}

#[derive(Debug, Fail)]
pub enum ParseError {
    #[fail(display = "Xpath error: {}", _0)]
    Xpath(#[cause] XpathError),
    #[fail(display = "Failed to parse field: {}", _0)]
    FieldParseFail(Error),
    #[fail(display = "XML errors at location {}: {:?}", location, errors)]
    InvalidXml {
        location: usize,
        errors: Vec<XmlError>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE: &str = r#"
        <?xml version="1.0" encoding="utf-8" ?>
        <!-- Copyright 2014-2018 First Lastname <your@email.com>, Blah <thing@blah.org> -->
        <component type="desktop-application">
            <name>Package</name>
            <id>org.foo.bar</id>
            <summary>Does something amazing</summary>
            <pkgname>blah</pkgname>
            <metadata_license>MIT OR Apache-2.0</metadata_license>
            <icon type="local">/usr/share/icon.png</icon>
        </component>
    "#;

    #[test]
    fn parse_simple() {
        let metainfo = Metainfo::from_str(SIMPLE);
        let thing = metainfo.validate().expect("Failed to read metainfo");

        println!("{:?}", thing);
    }
}
