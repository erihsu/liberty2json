use crate::{liberty_parser, LibertyJson};

use nom::{error::convert_error, Err};
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    str::FromStr,
};

pub struct Liberty {
    pub name: String,
    pub single_attribute: LibertyJson,
    pub group_attribute: LibertyJson,
    pub cell: HashMap<String, LibertyJson>,
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
