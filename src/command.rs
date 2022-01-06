/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use crate::eval::CommandError;

pub trait Builtin {
    fn name() -> &'static str;

    fn about() -> &'static str;

    fn examples() -> [&'static str; 3];

    fn run(args: Vec<String>) -> Result<(), CommandError>;

    fn help(&self) -> String {
        format!(
            "{}\nAbout: \n{}\nExamples: \n1. {}\n2. {}\n3. {}",
            Self::name(),
            Self::about(),
            Self::examples()[0],
            Self::examples()[1],
            Self::examples()[2]
        )
    }
}
