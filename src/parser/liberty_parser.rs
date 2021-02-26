use super::{
    attribute_parser::*,
    base::{lib_comment, tstring, ws},
    group_parser::*,
};
use crate::{ast::LibraryType, LibRes};

use nom::branch::alt;

use nom::{
    bytes::complete::tag,
    multi::many0,
    sequence::{delimited, preceded, tuple},
};

pub fn liberty_parser(input: &str) -> LibRes<&str, LibraryType> {
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
                many0(named_group_parser),
            )),
            ws(tag("}")),
        ),
    ))(input)
    .map(|(res, data)| {
        (
            res,
            LibraryType {
                name: data.1.to_string(),
                library: json::JsonValue::Array((data.2).0),
                cell: (data.2).1,
            },
        )
    })
}
