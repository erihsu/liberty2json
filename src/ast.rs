use crate::{liberty_parser, LibertyJson};

use nom::{error::convert_error, Err};
use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

#[derive(Clone)]
pub struct LibraryType {
    pub name: String,
    pub lib_attribute: LibertyJson,
}

pub struct CellType {
    pub name: String,
    pub cell_attribute: LibertyJson,
}

pub struct Liberty {
    pub library: LibraryType,
    pub cell: Vec<CellType>,
}

impl FromStr for Liberty {
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
