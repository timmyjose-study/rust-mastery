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
