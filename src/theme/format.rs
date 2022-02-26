/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use super::{parser::{Color, Node, Theme}, context::ThemeContext};
use colored::Colorize;

pub fn construct_colored<T: ThemeContext>(theme: &Theme, context: T) -> String {
    let mut colored = String::from("");
    let mut current_color = Color::new(255, 255, 255);
    for node in theme.get_vec().iter() {
        match node {
            Node::Var(v) => {
                let s = String::from(context.get_var(v.var_name));
                let s = s.truecolor(current_color.red, current_color.green, current_color.blue);
                colored = format!("{}{}", colored, s);
            }
            Node::Color(c) => {
                current_color = c.color.clone();
            }
            Node::Literal(l) => {
                let s = String::from(l.literal);
                let s = s.truecolor(current_color.red, current_color.green, current_color.blue);
                colored = format!("{}{}", colored, s);
            }
        }
    }

    colored
}
