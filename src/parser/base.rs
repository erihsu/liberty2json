use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, one_of};
use nom::combinator::{map, map_res, opt, recognize, value};
use nom::error::context;

use nom::multi::{many0, many1, separated_list1};

use crate::LibRes;
use json::JsonValue;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use std::str;
use std::str::FromStr;

// basic parse. Independent from def_parser but it's the most basic parser in def_parser.

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> LibRes<&'a str, O>
where
    F: FnMut(&'a str) -> LibRes<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

// // typical string
// // ie. abcdef, de234, jkl_mn, ...
pub fn tstring(input: &str) -> LibRes<&str, &str> {
    ws(recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    )))(input)
}

// quoted string, can include all ASCII character
pub fn qstring(input: &str) -> LibRes<&str, &str> {
    ws(delimited(tag("\""), take_until("\""), tag("\"")))(input)
}

// un-typical string, specific to unit
pub fn ustring(input: &str) -> LibRes<&str, &str> {
    ws(recognize(pair(number, unit_tag)))(input)
}

pub fn unit_tag(input: &str) -> LibRes<&str, &str> {
    recognize(pair(
        opt(one_of("kmunpf")),
        alt((
            tag("m"),
            tag("ohm"),
            tag("A"),
            tag("V"),
            tag("W"),
            tag("f"),
            tag("s"),
        )),
    ))(input)
}

// parse unsigned floating number
// The following is adapted from the Python parser by Valentin Lorentz (ProgVal).
pub fn float(input: &str) -> LibRes<&str, f32> {
    ws(map_res(
        alt((
            // Case one: .42
            recognize(tuple((
                char('.'),
                decimal,
                opt(tuple((one_of("eE"), opt(one_of("+-")), decimal))),
            ))), // Case two: 42e42 and 42.42e42
            recognize(tuple((
                opt(char('-')),
                decimal,
                opt(preceded(char('.'), decimal)),
                one_of("eE"),
                opt(one_of("+-")),
                decimal,
            ))), // Case three: 42. and 42.42
            recognize(tuple((opt(char('-')), decimal, char('.'), opt(decimal)))),
            recognize(tuple((opt(char('-')), decimal))), // case four: integer representation of float number
        )),
        |res: &str| f32::from_str(res),
    ))(input)
}

// lookup table value and its index
pub fn float_list(input: &str) -> LibRes<&str, Vec<f32>> {
    delimited(
        ws(tag("\"")),
        separated_list1(
            tuple((ws(tag(",")), ws(tag("\\")))),
            separated_list1(tag(","), float),
        ),
        ws(tag("\"")),
    )(input)
    .map(|(res, mut data)| {
        let mut float_list = Vec::new();
        for ele in data.iter_mut() {
            float_list.append(ele);
        }
        (res, float_list)
    })
}

pub fn float_list_no_breakline(input: &str) -> LibRes<&str, Vec<f32>> {
    delimited(
        ws(tag("\"")),
        separated_list1(tag(","), float),
        ws(tag("\"")),
    )(input)
}

pub fn float_array(input: &str) -> LibRes<&str, Vec<Vec<f32>>> {
    separated_list1(ws(tag(",\\")), float_list_no_breakline)(input)
}

pub fn number(input: &str) -> LibRes<&str, i32> {
    ws(map_res(
        recognize(pair(opt(alt((tag("+"), tag("-")))), digit1)),
        |res: &str| i32::from_str(res),
    ))(input)
}

pub fn decimal(input: &str) -> LibRes<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}

pub fn lib_comment(input: &str) -> LibRes<&str, ()> {
    value(
        (), // Output is thrown away.
        tuple((ws(tag("/*")), take_until("*/"), ws(tag("*/")))),
    )(input)
}

pub fn simple_attribute_value(input: &str) -> LibRes<&str, JsonValue> {
    alt((
        map(qstring, |s| JsonValue::from(s)),
        map(tstring, |s| JsonValue::from(s)),
        map(ustring, |s| JsonValue::from(s)),
        map(float, |s| JsonValue::from(s)),
    ))(input)
}

pub fn complex_attribue_value(input: &str) -> LibRes<&str, JsonValue> {
    delimited(
        ws(tag("(")),
        alt((
            map(float_array, |res| JsonValue::from(res)),
            map(float_list_no_breakline, |res| JsonValue::from(res)),
            map(float_list, |res| JsonValue::from(res)),
            map(separated_pair(tstring, tag(","), float), |res| {
                let mut json_obj = JsonValue::new_object();
                json_obj[res.0] = res.1.into();
                json_obj
            }),
            map(separated_pair(float, tag(","), float), |res| {
                JsonValue::Array(vec![JsonValue::from(res.0), JsonValue::from(res.1)])
            }),
            map(
                recognize(separated_pair(number, tag(","), tstring)),
                |res| JsonValue::String(res.to_string()),
            ),
            map(tstring, |res| JsonValue::String(res.to_string())),
        )),
        ws(tag(")")),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_array() {
        let input = "          \"-0.057255, -0.024717, 0.060606, 0.17141, 0.28867\",\\
                 \"-0.087193, -0.058049, 0.028245, 0.13813, 0.25745\",\\
                 \"-0.19269, -0.16383, -0.083466, 0.025617, 0.14229\",\\
                 \"-0.31589, -0.2968, -0.23804, -0.1371, -0.025855\",\\
                 \"-0.44037, -0.43037, -0.40215, -0.3206, -0.21397\"";
        let (_, _) = float_array(input).unwrap();
    }
    #[test]
    fn test_float_list_1() {
        let input = "\"0.01, 0.096347, 0.39944, 0.97825, 1.88\"";
        let (_, _) = float_list(input).unwrap();
    }

    #[test]
    fn test_float_list_2() {
        let input = "\"-0.087193, -0.058049, 0.028245, 0.13813, 0.25745\"";
        let (_, _) = float_list(input).unwrap();
    }
}
