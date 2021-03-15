use super::base::{complex_attribue_value, simple_attribute_value, tstring, ws};
use crate::{LibRes, LibertyJson};
use nom::{
    branch::alt,
    bytes::complete::tag,
    error::context,
    sequence::{separated_pair, terminated, tuple},
};

fn simple_attribute(input: &str) -> LibRes<&str, (&str, LibertyJson)> {
    terminated(
        separated_pair(tstring, tag(":"), simple_attribute_value),
        ws(tag(";")),
    )(input)
}

fn complex_attribute(input: &str) -> LibRes<&str, (&str, LibertyJson)> {
    terminated(tuple((tstring, complex_attribue_value)), ws(tag(";")))(input)
}

pub fn attribute_parser(input: &str) -> LibRes<&str, (&str, LibertyJson)> {
    context(
        "Attribute Parser",
        alt((complex_attribute, simple_attribute)),
    )(input)
}
