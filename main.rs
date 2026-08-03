// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

use derive_debug::CustomDebug;
use std::any;
use std::fmt::{Debug, Formatter};
use sorted::sorted;
//
// pub trait Trait {
//     type Value;
// }
//
// #[derive(CustomDebug)]
// pub struct Field<T: Trait> {
//     values: Vec<T::Value>,
// }
//
// fn assert_debug<F: Debug>() {}
//
// fn main() {
//     // Does not implement Debug, but its associated type does.
//     struct Id;
//
//     impl Trait for Id {
//         type Value = u8;
//     }
//
//     assert_debug::<Field<Id>>();
// }

#[sorted]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

enum ErrorKind {
    Io,
    Syntax,
    Eof,
}

fn main() {}
