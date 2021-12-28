/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

mod command;
mod commands;
mod eval;
mod prompt;
mod repl;
mod utils;

use repl::Repl;

fn main() {
    Repl::new().start_shell().unwrap();
}
