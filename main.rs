#![feature(proc_macro_hygiene, stmt_expr_attributes)]
// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run
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
use std::fmt::{self, Display};
use std::io;

#[sorted]
pub enum Error {
    Fmt(fmt::Error),
    Io(io::Error),
}

impl Display for Error {
    #[sorted::check]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        #[sorted]
        match self {
            Fmt(e) => write!(f, "{}", e),
            Io(e) => write!(f, "{}", e),
        }
    }
}

fn main() {}
