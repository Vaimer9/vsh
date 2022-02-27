/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use colored::Styles;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while1, take_while_m_n};
use nom::character::complete::one_of;
use nom::combinator::{eof, map_res};
use nom::multi::many_till;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use nom_locate::{position, LocatedSpan};

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

fn from_hex(input: Span) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(&input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: Span) -> IResult<Span, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

#[derive(PartialEq, Debug, Clone)]
pub struct ColorToken<'a> {
    pub end_pos: Span<'a>,
    pub color: Color,
}

#[derive(PartialEq, Debug, Clone)]
pub struct BackgroundColorToken<'a> {
    pub end_pos: Span<'a>,
    pub background_color: Option<Color>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct StyleToken<'a> {
    pub end_pos: Span<'a>,
    pub style: Styles,
}

/// Match against cbdurilhs
pub fn char_to_style(ch: char) -> Styles {
    match ch {
        'c' => Styles::Clear,
        'b' => Styles::Bold,
        'd' => Styles::Dimmed,
        'u' => Styles::Underline,
        'r' => Styles::Reversed,
        'i' => Styles::Italic,
        'l' => Styles::Blink,
        'h' => Styles::Hidden,
        's' => Styles::Strikethrough,
        _ => Styles::Clear,
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct VarToken<'a> {
    pub end_pos: Span<'a>,
    pub var_name: &'a str,
}

#[derive(PartialEq, Debug, Clone)]
pub struct LiteralToken<'a> {
    pub end_pos: Span<'a>,
    pub literal: &'a str,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Node<'a> {
    Var(VarToken<'a>),
    Color(ColorToken<'a>),
    BackgroundColor(BackgroundColorToken<'a>),
    Style(StyleToken<'a>),
    Literal(LiteralToken<'a>),
}

#[derive(Clone)]
pub struct Theme<'a>(Vec<Node<'a>>);

impl<'a> Theme<'a> {
    pub fn get_vec(&self) -> &Vec<Node<'a>> {
        &self.0
    }
}

#[cfg(test)]
impl<'a> Node<'a> {
    pub fn var(&self) -> Option<&VarToken<'a>> {
        match self {
            Node::Var(c) => Some(c),
            _ => None,
        }
    }

    pub fn color(&self) -> Option<&ColorToken<'a>> {
        match self {
            Node::Color(d) => Some(d),
            _ => None,
        }
    }

    pub fn background_color(&self) -> Option<&BackgroundColorToken<'a>> {
        match self {
            Node::BackgroundColor(d) => Some(d),
            _ => None,
        }
    }

    pub fn style(&self) -> Option<&StyleToken<'a>> {
        match self {
            Node::Style(d) => Some(d),
            _ => None,
        }
    }

    pub fn literal(&self) -> Option<&LiteralToken<'a>> {
        match self {
            Node::Literal(d) => Some(d),
            _ => None,
        }
    }
}

pub fn parse_color(s: Span) -> IResult<Span, Node> {
    let (s, _) = tag("&[#")(s)?;
    let (s, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(s)?;
    let (s, _) = tag("]")(s)?;
    let (s, end_pos) = position(s)?;

    Ok((
        s,
        Node::Color(ColorToken {
            end_pos,
            color: Color { red, green, blue },
        }),
    ))
}

pub fn parse_background_color(s: Span) -> IResult<Span, Node> {
    let (s, _) = tag("*[#")(s)?;
    let (s, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(s)?;
    let (s, _) = tag("]")(s)?;
    let (s, end_pos) = position(s)?;

    Ok((
        s,
        Node::BackgroundColor(BackgroundColorToken {
            end_pos,
            background_color: Some(Color { red, green, blue }),
        }),
    ))
}

pub fn parse_no_bg_color(s: Span) -> IResult<Span, Node> {
    let (s, _) = tag("*[]")(s)?;
    let (s, end_pos) = position(s)?;

    Ok((
        s,
        Node::BackgroundColor(BackgroundColorToken {
            end_pos,
            background_color: None,
        }),
    ))
}

pub fn parse_style(s: Span) -> IResult<Span, Node> {
    let (s, _) = tag("$[")(s)?;
    let (s, style_char) = one_of("cbdurilhs")(s)?;
    let (s, _) = tag("]")(s)?;
    let (s, end_pos) = position(s)?;

    Ok((
        s,
        Node::Style(StyleToken {
            end_pos,
            style: char_to_style(style_char),
        }),
    ))
}

pub fn parse_var(s: Span) -> IResult<Span, Node> {
    let (s, _) = tag("{{")(s)?;
    let (s, var_name) = take_until("}}")(s)?;
    let (s, _) = tag("}}")(s)?;
    let (s, end_pos) = position(s)?;

    Ok((
        s,
        Node::Var(VarToken {
            end_pos,
            var_name: &var_name,
        }),
    ))
}

pub fn parse_literal(s: Span) -> IResult<Span, Node> {
    let (s, literal) = delimited(tag("`"), take_while1(|c| c != '`'), tag("`"))(s)?;
    let (s, end_pos) = position(s)?;

    Ok((
        s,
        Node::Literal(LiteralToken {
            end_pos,
            literal: &literal,
        }),
    ))
}

pub fn parse_frag(s: Span) -> IResult<Span, Node> {
    let (s, n) = alt((
        parse_color,
        parse_var,
        parse_literal,
        parse_style,
        parse_no_bg_color,
        parse_background_color,
    ))(s)?;
    Ok((s, n))
}

pub fn parse_theme(s: Span) -> IResult<Span, Theme> {
    let (s, v) = many_till(parse_frag, eof)(s)?;
    Ok((s, Theme(v.0)))
}
