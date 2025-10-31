use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{alpha1, char, none_of},
    combinator::{map, opt, value},
    multi::{fold_many0, many0},
    sequence::{delimited, preceded},
};

use crate::{error::CommandError, request::Request};

pub struct Command {
    pub request: Request,
    pub args: Args,
}

#[derive(Debug)]
pub struct Args(Vec<String>);

impl Args {
    pub fn contains(&self, term: impl ToString) -> bool {
        let term = term.to_string().to_lowercase();
        self.0.iter().any(|s| s.to_lowercase() == term)
    }

    pub fn into_iter(self) -> impl Iterator<Item = String> {
        self.0.into_iter()
    }
}

impl Command {
    pub fn parse(source: &str) -> Result<Self, CommandError> {
        let (remaining, request) = match parse_request(source) {
            Ok((remaining, request)) => (remaining, request),
            Err(e) => return Err(CommandError::ParseError(e.to_string())),
        };

        let args = parse_args(remaining)?;

        Ok(Command { request, args })
    }
}

fn parse_request(input: &str) -> IResult<&str, Request> {
    use std::str::FromStr;

    let (input, request_str) = alpha1(input)?;
    let request = Request::from_str(request_str).map_err(|_| {
        nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::MapRes))
    })?;

    Ok((input, request))
}

fn parse_args(input: &str) -> Result<Args, CommandError> {
    let input = input.trim_start_matches(|c| c == ' ' || c == '\t');
    let (_, args) = many0(preceded(
        opt(take_while1(|c| c == ' ' || c == '\t')),
        parse_arg,
    ))
    .parse(input)
    .map_err(|e| CommandError::ParseError(e.to_string()))?;

    Ok(Args(args))
}

fn parse_escaped_char(input: &str) -> IResult<&str, char> {
    let escaped = alt((
        value('\\', char('\\')),
        value('"', char('"')),
        value('\'', char('\'')),
        value('\n', char('n')),
        value('\r', char('r')),
        value('\t', char('t')),
    ));

    let double_slash = char('\\');
    preceded(double_slash, escaped).parse(input)
}

fn parse_quoted_string(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        fold_many0(
            alt((parse_escaped_char, none_of("\"\\"))),
            String::new,
            |mut acc, c| {
                acc.push(c);
                acc
            },
        ),
        char('"'),
    )
    .parse(input)
}

fn parse_unquoted_arg(input: &str) -> IResult<&str, String> {
    map(
        take_while1(|c| c != ' ' && c != '\t' && c != '\n' && c != '\r'),
        |s: &str| s.to_string(),
    )
    .parse(input)
}

fn parse_arg(input: &str) -> IResult<&str, String> {
    alt((parse_quoted_string, parse_unquoted_arg)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_binarylimit() {
        let input = "binarylimit 5242880\n";
        let (remaining, request) = parse_request(input).unwrap();
        assert_eq!(request, Request::Binarylimit);
        assert_eq!(remaining, " 5242880\n");
    }
}
