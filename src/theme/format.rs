/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use super::{
    context::ThemeContext,
    parser::{Color, Node, Theme},
};
use colored::{ColoredString, Colorize, Styles};

fn apply_style(s: String, style: Styles) -> ColoredString {
    match style {
        Styles::Clear => s.normal(),
        Styles::Bold => s.bold(),
        Styles::Dimmed => s.dimmed(),
        Styles::Underline => s.underline(),
        Styles::Reversed => s.reversed(),
        Styles::Italic => s.italic(),
        Styles::Blink => s.blink(),
        Styles::Hidden => s.hidden(),
        Styles::Strikethrough => s.strikethrough(),
    }
}

pub fn construct_colored<T: ThemeContext>(theme: &Theme, context: T) -> String {
    let mut colored = String::from("");
    let mut current_color = Color::new(255, 255, 255);
    let mut current_background_color: Option<Color> = None;
    let mut current_style = Styles::Clear;

    for node in theme.get_vec().iter() {
        match node {
            Node::Var(v) => {
                let s = String::from(context.get_var(v.var_name));

                let s = apply_style(s, current_style);
                let s = s.truecolor(current_color.red, current_color.green, current_color.blue);
                let s = if let Some(bg) = current_background_color.as_ref() {
                    s.on_truecolor(bg.red, bg.green, bg.blue)
                } else {
                    s
                };

                colored = format!("{}{}", colored, s);
            }
            Node::Color(c) => {
                current_color = c.color.clone();
            }
            Node::Literal(l) => {
                let s = String::from(l.literal);

                let s = apply_style(s, current_style);
                let s = s.truecolor(current_color.red, current_color.green, current_color.blue);
                let s = if let Some(bg) = current_background_color.as_ref() {
                    s.on_truecolor(bg.red, bg.green, bg.blue)
                } else {
                    s
                };

                colored = format!("{}{}", colored, s);
            }
            Node::BackgroundColor(c) => current_background_color = c.background_color.clone(),
            Node::Style(s) => current_style = s.style,
            Node::Newline => (),
        }
    }

    colored
}
