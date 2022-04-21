//! Miscellaneous Rust concepts

mod impl_traits {
    use std::fmt::Debug;

    trait Trait {
        fn method(&self);
    }

    impl Trait for i32 {
        fn method(&self) {
            println!("Got an i32, {}", *self);
        }
    }

    impl Trait for f32 {
        fn method(&self) {
            println!("Got an f32, {}", *self);
        }
    }

    fn foo_dynamic() -> Box<dyn Trait> {
        Box::new(5)
    }

    fn foo_static() -> impl Trait + Debug {
        5
    }

    fn inc_dynamic() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    fn inc_static() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }

    fn bar_static() -> impl Iterator<Item = i32> {
        vec![1, 2, 3, 4, 5]
            .into_iter()
            .map(|x| x + 1)
            .filter(|x| x % 2 == 0)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_box_trait() {
            foo_dynamic().method();
        }

        #[test]
        fn test_impl_trait() {
            println!("{:?}", foo_static());
        }

        #[test]
        fn test_inc_dynamic() {
            assert_eq!(inc_dynamic()(99), 100);
        }

        #[test]
        fn test_inc_static() {
            assert_eq!(inc_static()(99), 100);
        }

        #[test]
        fn test_bar_static() {
            let mut iter = bar_static();
            while let Some(val) = iter.next() {
                print!("{} ", val);
            }
            println!();
        }
    }
}
