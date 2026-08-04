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


#[sorted::check]
fn f(bytes: &[u8]) -> Option<u8> {
    #[sorted]
    match bytes {
        [a] => Some(*a),
        [a, b] => Some(a + b),
        _other => None,
    }
}

fn main() {

    println!("{:?}", f(b"Hello, world!"));
}
