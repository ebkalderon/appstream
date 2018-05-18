//! Specifies the kind of application being packaged.

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ComponentType {
    DesktopApp,
    ConsoleApp,
    WebApp,
    Service,
    Addon,
    Font,
    Codec,
    InputMethod,
    Firmware,
    Driver,
    Localization,
}

impl FromStr for ComponentType {
    type Err = InvalidComponentType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "desktop" | "desktop-application" => Ok(ComponentType::DesktopApp),
            "console-application" => Ok(ComponentType::ConsoleApp),
            "web-application" => Ok(ComponentType::WebApp),
            "service" => Ok(ComponentType::Service),
            "addon" => Ok(ComponentType::Addon),
            "font" => Ok(ComponentType::Font),
            "codec" => Ok(ComponentType::Codec),
            "inputmethod" => Ok(ComponentType::InputMethod),
            "firmware" => Ok(ComponentType::Firmware),
            "driver" => Ok(ComponentType::Driver),
            "localization" => Ok(ComponentType::Localization),
            _ => Err(InvalidComponentType(s.to_string())),
        }
    }
}

impl Display for ComponentType {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            ComponentType::DesktopApp => fmt.write_str("desktop-application"),
            ComponentType::ConsoleApp => fmt.write_str("console-application"),
            ComponentType::WebApp => fmt.write_str("web-application"),
            ComponentType::Service => fmt.write_str("service"),
            ComponentType::Addon => fmt.write_str("addon"),
            ComponentType::Font => fmt.write_str("font"),
            ComponentType::Codec => fmt.write_str("codec"),
            ComponentType::InputMethod => fmt.write_str("inputmethod"),
            ComponentType::Firmware => fmt.write_str("firmware"),
            ComponentType::Driver => fmt.write_str("driver"),
            ComponentType::Localization => fmt.write_str("localization"),
        }
    }
}

#[derive(Clone, Debug, Eq, Fail, PartialEq)]
#[fail(display = "Invalid component `type` attribute: {}", _0)]
pub struct InvalidComponentType(String);
