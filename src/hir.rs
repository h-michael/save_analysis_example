#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
fn main() {
    <Person>::new("not_bind", 18);
    let kiske = <Person>::new("kiske", 18);
}

struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }
}
