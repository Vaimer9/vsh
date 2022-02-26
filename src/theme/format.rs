use std::collections::HashMap;

use colored::Colorize;

use super::parser::{Color, Node};

pub trait ThemeContext {
    fn get_var(&self, var_name: &str) -> &str;
}

pub struct Context {
    data: HashMap<String, String>,
}

impl ThemeContext for Context {
    fn get_var(&self, var_name: &str) -> &str {
        self.data.get(var_name).unwrap()
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set_var(&mut self, var_name: &str, value: &str) {
        self.data.insert(var_name.to_string(), value.to_string());
    }
}

pub fn construct_colored<T: ThemeContext>(theme: Vec<Node>, context: T) -> String {
    let mut colored = String::from("");
    let mut current_color = Color::new(255, 255, 255);
    for node in theme.iter() {
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
