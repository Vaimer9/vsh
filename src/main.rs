/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

mod builtins;
mod command;
mod eval;
mod prompt;
mod repl;
mod theme;
mod utils;

// use repl::Repl;
use parser::*;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    println!("{:#?}", parse_from_string(src));
}
