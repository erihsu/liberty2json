use super::{
    attribute_parser::*,
    base::{lib_comment, tstring, ws},
    group_parser::*,
};
use crate::{ast::LibraryType, CellType, LibRes, Liberty, LibertyJson};

use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::many0,
    sequence::{delimited, preceded, tuple},
};
use serde_json::map::Map;

pub fn liberty_parser(input: &str) -> LibRes<&str, Liberty> {
    tuple((
        many0(lib_comment),
        preceded(
            ws(tag("library")),
            delimited(ws(tag("(")), tstring, ws(tag(")"))),
        ),
        delimited(
            ws(tag("{")),
            tuple((
                many0(alt((attribute_parser, named_group_parser))),
                many0(cell_parser),
            )),
            ws(tag("}")),
        ),
    ))(input)
    .map(|(res, data)| {
        let mut json_data = Map::new();

        for attr_grp in (data.2).0 {
            json_data.insert(attr_grp.0.to_string(), attr_grp.1);
        }

        (
            res,
            Liberty {
                library: LibraryType {
                    name: data.1.to_string(),
                    lib_attribute: LibertyJson::from(json_data),
                },
                cell: (data.2).1,
            },
        )
    })
}

pub fn cell_parser(input: &str) -> LibRes<&str, CellType> {
    tuple((
        many0(lib_comment),
        preceded(
            ws(tag("cell")),
            delimited(ws(tag("(")), tstring, ws(tag(")"))),
        ),
        delimited(
            ws(tag("{")),
            many0(alt((
                attribute_parser,
                named_group_parser,
                unnamed_group_parser,
            ))),
            ws(tag("}")),
        ),
    ))(input)
    .map(|(res, data)| {
        let mut json_data = Map::new();
        for attr_grp in data.2 {
            json_data.insert(attr_grp.0.to_string(), attr_grp.1);
        }
        (
            res,
            CellType {
                name: data.1.to_string(),
                cell_attribute: LibertyJson::from(json_data),
            },
        )
    })
}
