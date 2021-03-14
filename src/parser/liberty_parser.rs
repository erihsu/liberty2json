use super::{
    attribute_parser::*,
    base::{lib_comment, tstring, ws},
    group_parser::*,
};
use crate::{ast::LibraryType, CellType, LibRes, Liberty};
use json::JsonValue;
use nom::branch::alt;

use nom::{
    bytes::complete::tag,
    multi::many0,
    sequence::{delimited, preceded, tuple},
};

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
        let mut json_data = JsonValue::new_object();
        for attr_grp in (data.2).0 {
            json_data[attr_grp.0] = attr_grp.1;
        }

        (
            res,
            Liberty {
                library: LibraryType {
                    name: data.1.to_string(),
                    lib_attribute: json_data,
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
        let mut json_data = JsonValue::new_object();
        for attr_grp in data.2 {
            json_data[attr_grp.0] = attr_grp.1;
        }
        (
            res,
            CellType {
                name: data.1.to_string(),
                cell_attribute: json_data,
            },
        )
    })
}
