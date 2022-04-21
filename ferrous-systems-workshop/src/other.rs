//! Miscellaneous Rust concepts.

mod r#macros {
    macro_rules! double {
        ($value: expr) => {
            $value * 2
        };
    }

    macro_rules! impl_foo_for {
        ($($typ: ty,)*) => {
            $(impl Foo for $typ {})*
        };

        ($($typ: ty),*) => {
            $(impl Foo for $typ {})*
        }
    }

    impl_foo_for!(i32, String, f32, f64,);
    impl_foo_for!(bool, i64);

    trait Foo {
        fn foo(&self) {
            println!("FOO!");
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_double() {
            println!("{}", double!(2 * 3 + 11));
            println!("{}", double![2 * 3 + 11]);
            println!("{}", double! {2 * 3 + 11});
        }

        #[test]
        fn test_foo() {
            100.foo();
            String::from("Hola, mundo!").foo();
            2.78128.foo();
        }
    }
}
