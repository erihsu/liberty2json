use super::{
    attribute_parser::*,
    base::{lib_comment, tstring, ws},
    group_parser::*,
};
use crate::{ast::LibraryType, CellType, LibRes, Liberty};

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
        (
            res,
            Liberty {
                library: LibraryType {
                    name: data.1.to_string(),
                    lib_attribute: (data.2).0,
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
        (
            res,
            CellType {
                name: data.1.to_string(),
                cell_attribute: data.2,
            },
        )
    })
}
