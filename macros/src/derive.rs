//! Custom Derive macros.

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Foo;

#[derive(HelloMacro)]
struct Person {
    name: String,
    age: u16,
}

mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        Foo::hello_macro();
    }

    #[test]
    fn test_person() {
        Person::hello_macro();
    }
}
