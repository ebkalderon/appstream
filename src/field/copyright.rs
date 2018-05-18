use regex::Regex;

use super::Field;

lazy_static! {
    static ref COPYRIGHT_COMMENT: Regex = Regex::new(r"Copyright [0-9]{4}((, |-)[0-9]{4})* .*")
        .expect("Invalid regular expression for validating copyright comments");
    static ref DATE_RANGE: Regex = Regex::new(r"[0-9]{4}((, |-)[0-9]{4})*")
        .expect("Invalid regular expression for selecting date ranges");
    static ref HOLDER_TEXT: Regex = Regex::new(r"Copyright [0-9]{4}((, |-)[0-9]{4})* ")
        .expect("Invalid regular expression for selecting non-holder text");
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Copyright {
    year: String,
    holder: String,
}

impl Field for Copyright {
    type Input = String;
    type Error = ParseError;

    const XPATH_EXPR: &'static str = "/comment()";

    fn construct(input: Self::Input) -> Result<Self, Self::Error> {
        let trimmed = input.trim();

        let year = DATE_RANGE
            .find(trimmed)
            .map(|s| s.as_str().to_string())
            .ok_or(ParseError::MalformedDateRange(trimmed.to_string()))?;

        let holder = HOLDER_TEXT.replace(trimmed, "").into_owned();

        if !COPYRIGHT_COMMENT.is_match(trimmed) {
            return Err(ParseError::InvalidCopyright(trimmed.to_string()));
        }

        Ok(Copyright { year, holder })
    }
}

#[derive(Clone, Debug, Fail)]
pub enum ParseError {
    #[fail(
        display = "Expected copyright comment, e.g. <!-- Copyright YYYY First Last <maintainer@email.org> -->"
    )]
    MissingCopyright,
    #[fail(display = "Malformed date range: {}", _0)]
    MalformedDateRange(String),
    #[fail(display = "Invalid copyright comment: {}", _0)]
    InvalidCopyright(String),
}
