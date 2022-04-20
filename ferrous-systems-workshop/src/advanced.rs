//! Advanced Rust

mod traits {
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        // associated method
        pub fn new(x: i32, y: i32) -> Self {
            Point { x, y }
        }
    }

    trait Distance<OtherShape = Point> {
        fn distance(&self, other: &OtherShape) -> f64;
    }

    impl Distance for Point {
        fn distance(&self, other: &Point) -> f64 {
            let x_dist = (self.x - other.x).pow(2);
            let y_dist = (self.y - other.y).pow(2);

            ((x_dist + y_dist) as f64).sqrt()
        }
    }

    trait MyAddition<Other = Point> {
        type Output; // associated types

        fn add(&self, other: &Point) -> Self::Output;
    }

    impl MyAddition for Point {
        type Output = Point;

        fn add(&self, other: &Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    // impl TraitS are not allowed in trait methods
    trait Foo {}

    trait Bar {
        //fn bar(&self) -> impl Foo;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_point() {
            let p = Point::new(100, 200);
            assert_eq!(p, Point { x: 100, y: 200 });
        }

        #[test]
        fn test_point_distance() {
            let p1 = Point::new(100, 200);
            let p2 = Point::new(-200, -100);
            println!(
                "Distance of {:?} from {:?} = {:.3}",
                p1,
                p2,
                p1.distance(&p2)
            );

            println!(
                "Distance of {:?} from {:?} = {:.3}",
                p1,
                p2,
                <dyn Distance<Point>>::distance(&p1, &p2)
            );
        }

        #[test]
        fn test_point_addition() {
            let p1 = Point::new(100, 200);
            let p2 = Point::new(-100, -200);

            println!("{:?} + {:?} = {:?}", p1, p2, p1.add(&p2));
            println!("{:?} + {:?} = {:?}", p2, p1, p2.add(&p1));
        }

        #[test]
        fn test_impl_trait() {
            let v = vec![1, 2, 3, 4, 5];
            let iter = make_iter(&v);
            let mut iter = double(iter);

            while let Some(e) = iter.next() {
                println!("{}", e);
            }

            fn make_iter<'a>(v: &'a Vec<u8>) -> impl Iterator<Item = u8> + 'a {
                v.iter().map(|e| *e + 2)
            }

            fn double(iter: impl Iterator<Item = u8>) -> impl Iterator<Item = u8> {
                iter.map(|e| e * 2)
            }
        }
    }
}

mod dynamoc_dispatch {
    // close-ended dynamic dispatch using enumS
    #[derive(Debug)]
    enum Operation {
        Get,
        Set(String),
        Count,
    }

    fn execute(op: Operation) {
        match op {
            Operation::Get => println!("Gotcha"),
            Operation::Set(string) => println!("Setting {:?}", string),
            Operation::Count => println!("Calling the count functionality"),
        }
    }

    // same as above, just bound with the enum itself
    impl Operation {
        fn execute(&self) {
            match self {
                Operation::Get => println!("Gotcha"),
                Operation::Set(string) => println!("Setting {:?}", string),
                Operation::Count => println!("Calling the count functionality"),
            }
        }
    }

    // closure traits are object-safe
    fn make_adder(n: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |x| x + n)
    }

    // uses static dispatch
    fn make_adder_again(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n
    }

    use std::any::Any;
    use std::fmt::Debug;

    // trait objects "know" which type they are, but this fact is not explicitly exposed to the end
    // user.
    fn log<T: Debug + Any>(val: &T) {
        if let Some(string) = <dyn Any>::downcast_ref::<String>(val) {
            println!("[Logger] string = {:?}, len = {:?}", string, string.len());
        } else if let Some(int) = <dyn Any>::downcast_ref::<i32>(val) {
            println!("[Logger] integer {:?}", int);
        } else if let Some(boolean) = <dyn Any>::downcast_ref::<bool>(val) {
            println!("[Logger] boolean {:?}", boolean);
        } else {
            println!("[Logger] Some other object {:?}", val);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_enum_dispatch() {
            let op = Operation::Set("hola".to_string());
            execute(op);

            let op = Operation::Count;
            op.execute();
            println!("{:?}", op);
        }

        #[test]
        fn test_make_adder() {
            let add10 = make_adder(10);

            for num in 1..=10 {
                print!("{} ", add10(num));
            }
            println!();

            let add5 = make_adder(5);

            for num in 1..=10 {
                print!("{} ", add5(num));
            }
            println!();
        }

        #[test]
        fn test_downcast() {
            log(&String::from("Hello, world"));
            log(&1);
            log(&true);
            log(&2.78128);
        }
    }
}

mod advanced_generics {
    use std::fmt::Debug;

    #[derive(Debug)]
    struct Foo;

    #[derive(Debug)]
    enum Bar {
        Baz,
        Quux(i32),
    }

    // using trait bounds to constrains generic types
    fn print_debug<T: Debug>(val: T) {
        println!("{:?}", val);
    }

    fn print_debug_again<T>(val: T)
    where
        T: Debug,
    {
        println!("{:?}", val);
    }

    trait Logger<T: Debug> {
        fn log(&self, t: T);
    }

    impl Logger<Bar> for Foo {
        fn log(&self, t: Bar) {
            println!("{:?}", t);
        }
    }

    // different constraints can be applied during construction as well as at call sites
    struct Wrapper<T> {
        inner: T,
    }

    impl<T> Wrapper<T> {
        pub fn new(inner: T) -> Self
        where
            T: Debug,
        {
            Wrapper { inner }
        }

        pub fn inspect(&self)
        where
            T: Debug,
        {
            println!("{:?}", self.inner);
        }
    }

    // generic trait implementations

    trait Shout {
        fn shout(&self);
    }

    impl<T: Debug> Shout for T {
        fn shout(&self) {
            println!("{:?} is shouting", *self);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_print_debug() {
            print_debug(100);
            print_debug(Bar::Baz);
            print_debug(Foo);

            print_debug_again(100);
            print_debug_again(Bar::Baz);
            print_debug_again(Foo);
        }

        #[test]
        fn test_logger() {
            let foo = Foo;
            foo.log(Bar::Baz);
        }

        #[test]
        fn test_wrapper() {
            let int_wrapper = Wrapper::new(100);
            int_wrapper.inspect();

            struct FooBar;

            // this is possible
            let _foobar_wrapper = Wrapper { inner: FooBar };

            // but not this since FooBar does not implement Debug
            //foobar_wrapper.inspect();
        }

        #[test]
        fn test_generic_trait_implementations() {
            100.shout();
            String::from("Hello, world").shout();
            2.278128.shout();
        }
    }
}
