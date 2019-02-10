#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std;
fn main() ({
               ((<Person>::new as
                    for<'r> fn(&'r str, u32) -> Person {Person::new})(("not_bind"
                                                                          as
                                                                          &'static str),
                                                                      (18 as
                                                                          u32))
                   as Person);
               let kiske =
                   ((<Person>::new as
                        for<'r> fn(&'r str, u32) -> Person {Person::new})(("kiske"
                                                                              as
                                                                              &'static str),
                                                                          (18
                                                                              as
                                                                              u32))
                       as Person);
           } as ())

struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    fn new(name: &str, age: u32)
     ->
         Person ({
                     (Person{name:
                                 ((name as &str).to_string() as
                                     std::string::String),
                             (age as u32),} as Person)
                 } as Person)
}
