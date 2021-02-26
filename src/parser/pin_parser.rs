use super::attribute_parser::attribute_parser;
use crate::LibRes;
use json::JsonValue;

use super::base_parser::{tstring, ws};

use nom::bytes::complete::tag;
use nom::multi::many1;
use nom::sequence::{delimited, preceded, terminated, tuple};
pub fn pin_parser(input: &str) -> LibRes<&str, JsonValue> {
    tuple((
        preceded(
            ws(tag("pin")),
            delimited(ws(tag("(")), tstring, ws(tag(")"))),
        ),
        delimited(
            ws(tag("{")),
            tuple((many1(attribute_parser), many1(pin_group_parser))),
            ws(tag("}")),
        ),
    ))(input)
    .map(|(res, data)| {
        let mut json_data = json::JsonValue::new_object();
        json_data["name"] = json::JsonValue::String(data.0.to_string());
        json_data["attribute"] = json::JsonValue::Array((data.1).0);
        json_data["group"] = json::JsonValue::Array((data.1).1);
        (res, json_data)
    })
}

fn pin_group_parser(input: &str) -> LibRes<&str, JsonValue> {
    tuple((
        terminated(tstring, ws(tag("()"))),
        delimited(ws(tag("{")), many1(attribute_parser), ws(tag("}"))),
    ))(input)
    .map(|(res, data)| {
        let mut json_data = json::JsonValue::new_object();
        json_data[data.0] = json::JsonValue::Array(data.1);
        (res, json_data)
    })
}
