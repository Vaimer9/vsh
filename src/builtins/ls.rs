// use crate::eval::CommandError;
//
// use crate::command::{expand, Builtin};
//
// pub struct Ls {
//     flags: Option<Vec<Flags>>
// }
//
// enum Flags {
//     L,
//     A
// }
//
// impl Builtin for Ls {
//     fn name() -> &'static str {
//         "ls"
//     }
//
//     fn about() -> &'static str {
//         "List content of current working directory"
//     }
//
//     fn examples() -> [&'static str; 3] {
//         ["ls", "ls -a", "ls -la"]
//     }
//
//     fn run(args: Vec<String>) -> Result<(), CommandError> {
//         
//     }
// }
