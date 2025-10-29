use std::str::FromStr;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, multispace0, multispace1},
    combinator::{map, recognize},
    multi::many0,
    sequence::{delimited, pair, separated_pair},
};
use strum::{Display, EnumString};

#[derive(Debug, EnumString, Display, PartialEq, Eq)]
#[strum(serialize_all = "snake_case", ascii_case_insensitive)]
pub enum TagName {
    Artist,
    ArtistSort,
    Album,
    AlbumSort,
    AlbumArtist,
    AlbumArtistSort,
    Title,
    TitleSort,
    Track,
    Name,
    Genre,
    Mood,
    Date,
    OriginalDate,
    Composer,
    ComposerSort,
    Performer,
    Conductor,
    Work,
    Ensemble,
    Movement,
    MovementNumber,
    ShowMovement,
    Location,
    Grouping,
    Comment,
    Disc,
    Label,
    MusicBrainzArtistId,
    MusicBrainzAlbumId,
    MusicBrainzAlbumArtistId,
    MusicBrainzTrackId,
    MusicBrainzReleaseGroupId,
    MusicBrainzReleaseTrackId,
    MusicBrainzWorkId,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    TagComparison {
        tag: TagName,
        op: TagOp,
        value: String,
    },
    FileComparison {
        op: FileOp,
        value: String,
    },
    AudioFormatComparison {
        op: AudioFormatOp,
        value: String,
    },
    PriorityComparison {
        op: PriorityOp,
        value: i32,
    },
    Not(Box<Expression>),
    And(Vec<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum TagOp {
    Eq,
    Neq,
    Contains,
    StartsWith,
    RegexMatch,
    RegexNotMatch,
}

#[derive(Debug, PartialEq)]
pub enum FileOp {
    Eq,
    Base,
    ModifiedSince,
    AddedSince,
}

#[derive(Debug, PartialEq)]
pub enum AudioFormatOp {
    Eq,
    RegexMatch,
}

#[derive(Debug, PartialEq)]
pub enum PriorityOp {
    Ge,
}

fn parse_tag_op(input: &str) -> IResult<&str, TagOp> {
    alt((
        map(tag("=="), |_| TagOp::Eq),
        map(tag("!="), |_| TagOp::Neq),
        map(tag("contains"), |_| TagOp::Contains),
        map(tag("starts_with"), |_| TagOp::StartsWith),
        map(tag("=~"), |_| TagOp::RegexMatch),
        map(tag("!~"), |_| TagOp::RegexNotMatch),
    ))
    .parse(input)
}

fn parse_file_op(input: &str) -> IResult<&str, FileOp> {
    alt((
        map(tag("=="), |_| FileOp::Eq),
        map(tag("base"), |_| FileOp::Base),
        map(tag("modified-since"), |_| FileOp::ModifiedSince),
        map(tag("added-since"), |_| FileOp::AddedSince),
    ))
    .parse(input)
}

fn parse_audio_format_op(input: &str) -> IResult<&str, AudioFormatOp> {
    alt((
        map(tag("=="), |_| AudioFormatOp::Eq),
        map(tag("=~"), |_| AudioFormatOp::RegexMatch),
    ))
    .parse(input)
}

fn parse_priority_op(input: &str) -> IResult<&str, PriorityOp> {
    map(tag(">="), |_| PriorityOp::Ge).parse(input)
}

fn parse_string_value(input: &str) -> IResult<&str, String> {
    map(
        delimited(char('\''), take_until("'"), char('\'')),
        |s: &str| s.to_string(),
    )
    .parse(input)
}

fn parse_tag_comparison(input: &str) -> IResult<&str, Expression> {
    map(
        (
            char('('),
            delimited(char('('), parse_tag_name, multispace0),
            parse_tag_op,
            multispace0,
            parse_string_value,
            delimited(multispace0, char(')'), multispace0),
            char(')'),
        ),
        |(_, tag, op, _, value, _, _)| Expression::TagComparison {
            tag: TagName::from_str(&tag).unwrap(),
            op,
            value,
        },
    )
    .parse(input)
}

fn parse_file_comparison(input: &str) -> IResult<&str, Expression> {
    map(
        (
            delimited(char('('), parse_file_op, multispace0),
            parse_string_value,
            delimited(multispace0, char(')'), multispace0),
        ),
        |(op, value, _)| Expression::FileComparison { op, value },
    )
    .parse(input)
}

fn parse_audio_format_comparison(input: &str) -> IResult<&str, Expression> {
    map(
        (
            delimited(char('('), tag("AudioFormat"), multispace0),
            parse_audio_format_op,
            multispace0,
            parse_string_value,
            delimited(multispace0, char(')'), multispace0),
        ),
        |(_, op, _, value, _)| Expression::AudioFormatComparison { op, value },
    )
    .parse(input)
}

fn parse_priority_comparison(input: &str) -> IResult<&str, Expression> {
    map(
        (
            delimited(char('('), tag("prio"), multispace0),
            parse_priority_op,
            multispace0,
            nom::character::complete::i32,
            delimited(multispace0, char(')'), multispace0),
        ),
        |(_, op, _, value, _)| Expression::PriorityComparison { op, value },
    )
    .parse(input)
}

fn parse_not_expression(input: &str) -> IResult<&str, Expression> {
    map(
        delimited(
            (char('('), multispace0, char('!')),
            parse_expression,
            delimited(multispace0, char(')'), multispace0),
        ),
        |expr| Expression::Not(Box::new(expr)),
    )
    .parse(input)
}

fn parse_and_expression(input: &str) -> IResult<&str, Expression> {
    map(
        delimited(
            char('('),
            separated_pair(
                parse_expression,
                delimited(multispace1, tag("AND"), multispace1),
                parse_expression,
            ),
            char(')'),
        ),
        |(first, second)| Expression::And(vec![first, second]),
    )
    .parse(input)
}

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        parse_tag_comparison,
        parse_file_comparison,
        parse_audio_format_comparison,
        parse_priority_comparison,
        parse_not_expression,
        parse_and_expression,
    ))
    .parse(input)
}

fn parse_tag_name(input: &str) -> IResult<&str, String> {
    map(
        recognize(pair(
            nom::character::complete::alpha1,
            many0(alt((nom::character::complete::alphanumeric1, tag("_")))),
        )),
        |s: &str| s.to_string(),
    )
    .parse(input)
}
