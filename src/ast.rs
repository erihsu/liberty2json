use crate::liberty_parser;
use json::JsonValue;

use nom::{error::convert_error, Err};
use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

#[derive(Clone)]
pub struct LibraryType {
    pub name: String,
    pub library: JsonValue,
    pub cell: Vec<JsonValue>,
}

impl FromStr for LibraryType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match liberty_parser(s) {
            Ok((_, u)) => Ok(u),
            Err(Err::Error(e)) => {
                println!("[LibParser] `VerboseError`:\n{}", convert_error(s, e));
                Err(Error::new(ErrorKind::InvalidData, "Invalid Liberty File"))
            }
            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid Liberty File")),
        }
    }
}
