use json::JsonValue;
use nom::bytes::complete::tag;

use crate::LibRes;
use nom::multi::many1;
use nom::sequence::{delimited, preceded, tuple};

use super::attribute_parser::attribute_parser;
use super::base_parser::{tstring, ws};
use super::pin_parser::pin_parser;
pub fn cell_parser(input: &str) -> LibRes<&str, JsonValue> {
    tuple((
        preceded(
            ws(tag("cell")),
            delimited(ws(tag("(")), tstring, ws(tag(")"))),
        ),
        delimited(
            ws(tag("{")),
            tuple((many1(attribute_parser), many1(pin_parser))),
            ws(tag("}")),
        ),
    ))(input)
    .map(|(res, data)| {
        let mut json_data = json::JsonValue::new_object();
        json_data["name"] = JsonValue::String(data.0.to_string());
        json_data["attribute"] = JsonValue::Array((data.1).0);
        json_data["pin"] = JsonValue::Array((data.1).1);
        (res, json_data)
    })
}
