use super::{
    attribute_parser::*,
    base_parser::{lib_comment, tstring, ws},
    cell_parser::*,
};
use crate::{ast::LibraryType, LibRes};
use json::JsonValue;

use nom::{
    bytes::complete::tag,
    multi::{many0, many1},
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
                many1(attribute_parser),
                many1(library_group_parser),
                many1(cell_parser),
            )),
            ws(tag("}")),
        ),
    ))(input)
    .map(|(res, data)| {
        (
            res,
            LibraryType {
                name: data.1.to_string(),
                attribute: json::JsonValue::Array((data.2).0),
                group: json::JsonValue::Array((data.2).1),
                cell: (data.2).2,
            },
        )
    })
}

fn library_group_parser(input: &str) -> LibRes<&str, JsonValue> {
    tuple((
        tstring,
        delimited(tag("("), tstring, tag(")")),
        delimited(ws(tag("{")), many1(attribute_parser), ws(tag("}"))),
    ))(input)
    .map(|(res, data)| {
        let mut json_data = json::JsonValue::new_object();
        json_data["key"] = JsonValue::String(data.0.to_string());
        json_data["name"] = JsonValue::String(data.1.to_string());
        json_data["attribute"] = json::JsonValue::Array(data.2);
        (res, json_data)
    })
}
