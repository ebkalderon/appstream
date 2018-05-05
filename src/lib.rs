//! Parser for AppStream metadata.

#[macro_use]
extern crate failure;
extern crate license_exprs;
#[macro_use]
extern crate serde;
extern crate serde_xml_rs;
extern crate sxd_document;
extern crate tld as tld_rs;
extern crate url;
#[macro_use]
extern crate url_serde;

mod id;
mod license;

use std::path::PathBuf;

use url::Url;

use id::Id;
use license::License;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Icon {
    Stock {
        id: String,
    },
    Local {
        path: PathBuf,
    },
    Remote { 
        url: Url,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppStream {
    id: Id,
    metadata_license: Option<License>,
    name: Option<String>,
    summary: Option<String>,
    icon: Option<Icon>,
}

#[cfg(test)]
mod tests {
    use serde_xml_rs::{deserialize, serialize};

    use super::*;

    const SIMPLE: &str = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <!-- Copyright 2013 First Lastname <your@email.com> -->
        <component type="desktop-application">
            <name>blah</name>
            <id>org.foo.bar</id>
            <metadata_license>MIT</metadata_license>
            <icon type="local">/usr/share/icon.png</icon>
        </component>
    "#;

    #[test]
    fn deserialize_simple() {
        let bytes: Vec<u8> = SIMPLE.bytes().collect();
        let actual: AppStream =
            deserialize(bytes.as_slice()).expect("Invalid AppStream specification");

        println!("{:#?}", actual);

        let mut buf = Vec::new();
        serialize(actual, &mut buf).expect("Unable to serialize");
        let mut s = String::from_utf8(buf).expect("Unable to parse string");
        println!("{}", s);
    }
}
