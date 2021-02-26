use super::base_parser::{complex_attribue_value, simple_attribute_value, tstring, ws};
use crate::LibRes;
use json::JsonValue;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::{separated_pair, terminated, tuple};

fn simple_attribute(input: &str) -> LibRes<&str, (&str, JsonValue)> {
    terminated(
        separated_pair(tstring, tag(":"), simple_attribute_value),
        ws(tag(";")),
    )(input)
}

fn complex_attribute(input: &str) -> LibRes<&str, (&str, JsonValue)> {
    terminated(tuple((tstring, complex_attribue_value)), ws(tag(";")))(input)
}

pub fn attribute_parser(input: &str) -> LibRes<&str, JsonValue> {
    alt((simple_attribute, complex_attribute))(input).map(|(res, data)| {
        let mut json_data = json::JsonValue::new_object();
        json_data[data.0] = data.1;
        (res, json_data)
    })
}
