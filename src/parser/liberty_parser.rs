use super::{
    attribute_parser::*,
    base::{lib_comment, tstring, ws},
    cell_parser::*,
    group_parser::*,
};
use crate::{LibRes, Liberty, LibertyJson};

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    error::context,
    multi::{many0, many1},
    sequence::{delimited, preceded, tuple},
};
use serde_json::map::Map;

enum LibertyElemEnum<'a> {
    Attribute((&'a str, LibertyJson)),
    Group((&'a str, LibertyJson)),
    Cell((&'a str, LibertyJson)),
}

use std::collections::HashMap;
pub fn liberty_parser(input: &str) -> LibRes<&str, Liberty> {
    context(
        "Liberty Parser",
        tuple((
            many0(lib_comment),
            preceded(
                ws(tag("library")),
                delimited(ws(tag("(")), tstring, ws(tag(")"))),
            ),
            delimited(
                ws(tag("{")),
                many1(alt((
                    map(cell_parser, |x| LibertyElemEnum::Cell(x)),
                    map(header_group_parser, |x| LibertyElemEnum::Group(x)),
                    map(header_attribute_parser, |x| LibertyElemEnum::Attribute(x)),
                ))),
                ws(tag("}")),
            ),
        )),
    )(input)
    .map(|(res, data)| {
        let mut attrs = Map::new();
        let mut grps = Map::new();
        let mut cells = HashMap::new();
        for d in data.2 {
            match d {
                LibertyElemEnum::Cell(u) => cells.insert(u.0.to_string(), u.1),
                LibertyElemEnum::Group(u) => grps.insert(u.0.to_string(), u.1),
                LibertyElemEnum::Attribute(u) => attrs.insert(u.0.to_string(), u.1),
            };
        }

        (
            res,
            Liberty {
                name: data.1.to_string(),
                single_attribute: LibertyJson::from(attrs),
                group_attribute: LibertyJson::from(grps),
                cell: cells,
            },
        )
    })
}
