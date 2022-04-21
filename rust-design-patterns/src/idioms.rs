//! Idiomatic Rust.

// Prefer borrowed types over owned types, so &str over &String, &[T] over &Vec<T> and &T over
// &Box<T> etc.
mod borrowed_types_for_arguments {
    fn three_consecutive_vowels(s: &str) -> bool {
        let mut vowel_count = 0;

        for c in s.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    vowel_count += 1;
                    if vowel_count == 3 {
                        return true;
                    }
                }
                _ => vowel_count = 0,
            }
        }
        false
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_three_consecutive_vowels() {
            let ferris = "ferris".to_string();
            assert!(!three_consecutive_vowels(&ferris));

            assert!(three_consecutive_vowels("hawaii"));
        }
    }
}

// Static strings can be concatenated at compile-time using the concat! macro.
// Runtime strings can be concatenated using the format! macro.
mod concatenating_strings_with_format {
    fn say_hello(name: &str) -> String {
        format!("{}, {}", "Hello", name)
    }

    fn say_hello_again() -> &'static str {
        concat!("Hello, ", "my name is ", "Bob.", " Nice to meet ", "you")
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_concat() {
            assert_eq!(say_hello_again(), "Hello, my name is Bob. Nice to meet you");
        }

        #[test]
        fn test_format() {
            assert_eq!(say_hello("Bob"), "Hello, Bob");
        }
    }
}

// Use associated functions as a poor man's substitute for constructors.
// Default constructors can be provided by implementing the Default trait.
pub mod constructors {
    use std::default::Default;

    /// Time in seconds.
    ///
    /// # Example
    ///
    /// ```
    /// use  rust_design_patterns::idioms::constructors::Second;
    ///
    /// let s = Second::new(42);
    /// assert_eq!(s.value(), 42);
    /// ```
    pub struct Second {
        value: i32,
    }

    impl Second {
        pub fn new(value: i32) -> Self {
            Second { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    impl Default for Second {
        fn default() -> Self {
            Second { value: 0 }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_second() {
            let four_score_and_seven = Second::new(87);
            assert_eq!(four_score_and_seven.value(), 87);
        }

        #[test]
        fn test_default_second() {
            assert_eq!(Second::default().value(), 0);
        }
    }
}

mod default_trait {
    use std::{path::PathBuf, time::Duration};

    #[derive(Debug, Default)]
    pub struct MyConfiguration {
        pub output: Option<PathBuf>,
        pub search_path: Vec<PathBuf>,
        pub timeout: Duration,
        pub check: bool,
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_defaults() {
            let config = MyConfiguration::default();
            assert_eq!(config.output, None);
            assert_eq!(config.search_path, vec![] as Vec<PathBuf>);
            assert_eq!(config.timeout, Duration::from_secs(0));
            assert_eq!(config.check, false);
        }
    }
}

// Use the Deref (and DerefMut, if needed) trait to provide better and more flexible interfaces for
// collections.
mod collections_are_smart_pointers {
    use std::ops::Deref;

    struct MyVec<T> {
        inner: Vec<T>,
    }

    impl<T> Deref for MyVec<T> {
        type Target = Vec<T>;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    fn take_borrowed_vec<T>(inp: &Vec<T>) {
        println!("len = {}", inp.len());
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_deref() {
            let myvec = MyVec {
                inner: vec![1, 2, 3, 4, 5],
            };

            take_borrowed_vec(&myvec);
            take_borrowed_vec(&vec!["hello", "world"]);
        }
    }
}

mod finalisation_in_destructors {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u16,
    }

    impl Person {
        pub fn new(name: &str, age: u16) -> Self {
            Person {
                name: name.into(),
                age,
            }
        }
    }

    impl Drop for Person {
        fn drop(&mut self) {
            println!("Adios, {:?}!", *self);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_drop() {
            let _bob = Person::new("Bob", 42);
            {
                let _dave = Person::new("Dave", 21);
                {
                    let _greg = Person::new("Greg", 99);
                    let _baz = Person::new("Baz", 10);
                }
            }
        }
    }
}

// Helps with avoiding the "using clone to please the Borrow Checker" anti-pattern.
mod mem_take_replace {
    enum Foo {
        A { name: String, x: i32 },
        B { name: String },
        C,
        D,
    }

    fn a_to_b(var: &mut Foo) {
        if let Foo::A { name, x: 0 } = var {
            *var = Foo::B {
                name: std::mem::take(name),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_mem_take() {
            let mut foo = Foo::A {
                name: "Bob".into(),
                x: 100,
            };

            a_to_b(&mut foo);

            if let Foo::A { ref name, ref x } = &foo {
            } else {
                assert!(false);
            }

            let mut foo_again = Foo::A {
                name: "Dave".into(),
                x: 0,
            };

            a_to_b(&mut foo_again);

            if let Foo::B { ref name } = &foo_again {
            } else {
                assert!(false);
            }
        }
    }
}

// this actually arises because Rust does not have a way to selectively move data into
// a closure.
mod pass_variables_to_closure {
    #[cfg(test)]
    mod tests {
        #[test]
        fn test_selective_move_into_closure() {
            use std::rc::Rc;

            let val1 = Rc::new(1);
            let val2 = Rc::new(2);
            let val3 = Rc::new(3);

            let closure = {
                // val1 is moved
                let val2 = val2.clone();
                let val3 = val3.as_ref(); // the reference is moved
                move || *val1 + *val2 + *val3
            };

            assert_eq!(closure(), 6);
            assert_eq!(*val2, 2);
            assert_eq!(*val3, 3);
        }
    }
}

// Option implements IntoIterator
mod iterating_over_option {
    #[cfg(test)]
    mod tests {
        #[test]
        fn test_option_iteration() {
            let turing = Some("Turing");
            let mut logicians = vec!["Curry", "Kleene", "Markov"];

            logicians.extend(turing);
            assert_eq!(logicians, &["Curry", "Kleene", "Markov", "Turing"]);

            let timmy = Some("Jose");
            for logician in logicians.iter().chain(timmy.iter()) {
                println!("{:?} is a logician", logician);
            }
        }
    }
}

mod temporary_mutability {
    #[cfg(test)]
    mod tests {
        #[test]
        fn test_temp_mut_block() {
            let data = {
                let mut data = vec![5, 1, 2, 3, 4];
                data.sort();
                data
            };

            assert_eq!(data, &[1, 2, 3, 4, 5]);
        }
        #[test]
        fn test_temp_mut_shadowing() {
            let mut data = vec![5, 2, 1, 4, 3];
            data.sort();
            let data = data;
            assert_eq!(data, &[1, 2, 3, 4, 5]);
        }
    }
}

// dynamic dispatch is usually handled by using Trait Objects, but we can use only stack-allocated
// values to get dynamic dispatch.
mod on_stack_dynamic_dispatch {
    use std::fs::File;
    use std::io::{self, Read};

    // the usual way of doing it - this allocates on the heap.
    fn dynamic_dispatch_using_box(inp: &str) -> Result<usize, io::Error> {
        let (mut stdin_read, mut file_read);

        let mut readable: Box<&mut dyn Read> = if inp == "-" {
            stdin_read = io::stdin();
            Box::new(&mut stdin_read)
        } else {
            file_read = File::open(inp)?;
            Box::new(&mut file_read)
        };

        let mut buffer = String::new();
        readable.read_to_string(&mut buffer)?;

        Ok(buffer.len())
    }

    // this is fully on the stack.
    fn dynamic_dispatch_using_stack(inp: &str) -> Result<usize, io::Error> {
        let (mut stdin_read, mut file_read);

        let readable: &mut dyn Read = if inp == "-" {
            stdin_read = io::stdin();
            &mut stdin_read
        } else {
            file_read = File::open(inp)?;
            &mut file_read
        };

        let mut buffer = String::new();
        readable.read_to_string(&mut buffer)?;

        Ok(buffer.len())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_dynamic_dispatch_using_box() {
            println!("{:?}", dynamic_dispatch_using_box("src/idioms.rs").unwrap());
            println!("{:?}", dynamic_dispatch_using_box("-").unwrap());
        }

        #[test]
        fn test_dynamic_dispatch_using_stack() {
            println!(
                "{:?}",
                dynamic_dispatch_using_stack("src/idioms.rs").unwrap()
            );
            println!("{:?}", dynamic_dispatch_using_stack("-").unwrap());
        }
    }
}
