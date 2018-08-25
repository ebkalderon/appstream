use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

use super::Field;

const CATEGORIES: &[&str] = &[
    "Audio",
    "AudioVideo",
    "Development",
    "Education",
    "Game",
    "Graphics",
    "Network",
    "Office",
    "Science",
    "Settings",
    "System",
    "Utility",
    "Video",
];

lazy_static! {
    static ref RELATED_CATEGORIES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("Building", vec!["Development"]);
        map.insert("Debugger", vec!["Development"]);
        map.insert("IDE", vec!["Development"]);
        map
    };
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Category(String);

impl FromStr for Category {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cat = CATEGORIES
            .iter()
            .find(|&cat| s == *cat)
            .ok_or(ParseError::UnknownCategory(s.into()))?;

        Ok(Category(cat.to_string()))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Categories(Vec<Category>);

impl Deref for Categories {
    type Target = [Category];

    fn deref(&self) -> &[Category] {
        let Categories(ref vec) = *self;
        vec.as_slice()
    }
}

impl Field for Option<Categories> {
    type Input = Vec<String>;
    type Error = ParseError;

    const XPATH_EXPR: &'static str = "/component/categories/category/text()";

    fn construct(input: Self::Input) -> Result<Self, Self::Error> {
        if input.len() == 0 {
            return Ok(None);
        }

        input
            .into_iter()
            .map(|value| Category::from_str(&value))
            .collect::<Result<_, _>>()
            .map(|parsed| Some(Categories(parsed)))
    }
}

#[derive(Debug, Fail)]
pub enum ParseError {
    #[fail(display = "unknown category: {}", _0)]
    UnknownCategory(String),
}
