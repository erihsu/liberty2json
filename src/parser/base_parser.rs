use nom::branch::alt;

use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, one_of};
use nom::combinator::{map, map_res, opt, recognize, value};

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
                decimal,
                opt(preceded(char('.'), decimal)),
                one_of("eE"),
                opt(one_of("+-")),
                decimal,
            ))), // Case three: 42. and 42.42
            recognize(tuple((decimal, char('.'), opt(decimal)))),
            recognize(decimal), // case four: integer representation of float number
        )),
        |res: &str| f32::from_str(res),
    ))(input)
}

pub fn float_list(input: &str) -> LibRes<&str, Vec<f32>> {
    delimited(
        ws(tag("\"")),
        separated_list1(tag(","), float),
        ws(tag("\"")),
    )(input)
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
        map(float, |s| JsonValue::from(s)),
        map(tstring, |s| JsonValue::from(s)),
    ))(input)
}

pub fn complex_attribue_value(input: &str) -> LibRes<&str, JsonValue> {
    delimited(
        ws(tag("(")),
        alt((
            map(separated_pair(float, tag(","), float), |res| {
                JsonValue::Array(vec![JsonValue::from(res.0), JsonValue::from(res.1)])
            }),
            map(separated_pair(number, tag(","), tstring), |res| {
                JsonValue::Array(vec![JsonValue::from(res.0), JsonValue::from(res.1)])
            }),
            map(float_list, |res| JsonValue::from(res)),
            map(
                separated_list1(tag(","), preceded(ws(tag("\\")), float_list)),
                |res| JsonValue::from(res),
            ),
        )),
        ws(tag(")")),
    )(input)
}
