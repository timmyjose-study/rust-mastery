//! Rust Basics

mod basic_tyoes {
    #[cfg(test)]
    mod tests {
        use std::any::Any;

        #[test]
        fn test_cast() {
            let foo = 3i64;
            let bar = foo as i32;
            assert!(<dyn Any>::is::<i32>(&bar));
        }

        #[test]
        fn test_default_integer_type() {
            let x = 100;

            assert!(<dyn Any>::is::<i32>(&x));
        }
    }
}

mod control_flow {
    use std::fs;
    use std::io;
    use std::io::Read;
    use std::path::Path;

    fn read_file(path: &Path) -> Result<String, io::Error> {
        let mut f = fs::File::open(path)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;

        Ok(buffer)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_overflow() {
            let maybe_overflow = 10u8.checked_add(250);
            assert_eq!(maybe_overflow, None);

            let maybe_overflow = 1u8.checked_add(250);
            assert_eq!(maybe_overflow, Some(251u8));
        }

        #[test]
        fn test_file_open() {
            let handle = std::fs::File::open("foo");
            assert!(handle.is_err());

            let handle = std::fs::File::open("src/basics.rs");
            assert!(handle.is_ok());
        }

        #[test]
        fn test_read_file() {
            println!(
                "{}",
                match read_file(Path::new("src/basics.rs")) {
                    Err(e) => e.to_string(),
                    Ok(contents) => contents,
                }
            );
        }
    }
}

mod error_handling {
    use std::error::Error;
    use std::fs::File;

    fn this_can_fail(succeeds: bool) -> Result<String, String> {
        if succeeds {
            Ok(String::from("Success"))
        } else {
            Err(String::from("failure"))
        }
    }

    fn multiple_possible_failures() -> Result<String, String> {
        println!("{}", this_can_fail(true)?);
        println!("After 1st potential error");
        println!("{}", this_can_fail(false)?);
        println!("After 2nd potential error");

        Ok(String::from("all good"))
    }

    fn convert_errors() -> Result<(), Box<dyn Error>> {
        let _f = File::open("src/basic.rs")?;
        let _x = "123abc".parse::<i32>()?;

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_error_basic() {
            assert!(this_can_fail(true).is_ok());
            assert!(this_can_fail(false).is_err());
        }

        #[test]
        fn test_error_multiple() {
            assert!(multiple_possible_failures().is_err());
        }

        #[test]
        fn test_error_mapping() {
            // map -> for success, map_err -> for error
            println!("{:?}", this_can_fail(true).map(|val| val.len()));
            println!(
                "{:?}",
                this_can_fail(false).map_err(|err_val| err_val.to_uppercase())
            );
        }

        #[test]
        fn test_dynamic_error() {
            println!(
                "{:?}",
                convert_errors().map_err(|err| err.to_string().to_uppercase())
            );
        }
    }
}

mod ownership {
    #[derive(Debug, Copy, Clone)]
    struct Dot {
        pub x: i32,
        pub y: i32,
    }

    fn pacman(dot: Dot) {
        println!("Eating {:?}", dot);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_eating() {
            let dot = Dot { x: 100, y: 200 };
            pacman(dot);

            // dot is consumed, so this would be invalid if
            //` Dot` were not `Copy`.
            //pacman(dot);

            let dot = Dot { x: -100, y: -200 };
            pacman(dot.clone());

            // this is fine, but this call consumes `dot`
            pacman(dot);

            // with copy, we don't have to worry about consuming it
            let dot = Dot { x: 0, y: -12345 };
            pacman(dot);
            pacman(dot);
            pacman(dot);
            pacman(dot);
            pacman(dot);
        }
    }
}

mod borrowing {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    fn inspect(p: &Point) {
        println!("{:?}", p);
    }

    fn move_point(p: &mut Point, x: i32, y: i32) {
        p.x = x;
        p.y = y;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_borrowing() {
            let mut p = Point { x: 100, y: 200 };
            inspect(&p);
            p.x = 12345;
            inspect(&p);
        }

        #[test]
        fn test_move_point() {
            let mut p = Point { x: -100, y: -200 };
            inspect(&p);
            move_point(&mut p, 0, 0);
            inspect(&p);
        }

        #[test]
        fn test_dereferencing() {
            let p = &mut 100;
            println!("p before = {}", *p);
            *p += 100;
            println!("p after = {}", *p);
        }
    }
}

mod strings {
    fn accept_either<S: AsRef<str> + ?Sized>(thing: &S) -> String {
        String::from("foo") + thing.as_ref()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_string_deref() {
            let part_one = String::from("Hello ");
            let part_two = String::from("world ");
            let whole = part_one + &part_two + "again";
            assert_eq!(whole, "Hello world again");
        }

        #[test]
        fn test_split_string() {
            let text = "Cow says moo";
            let words = text.split(" ").collect::<Vec<_>>();
            assert_eq!(words, ["Cow", "says", "moo"]);
        }

        #[test]
        fn test_string_concatenation() {
            let animal = String::from("Cow");
            let sound = String::from("moo");
            let text = [&animal, " says ", &sound].concat();
            assert_eq!(text, "Cow says moo");
        }

        #[test]
        fn test_string_replacement() {
            let text = "Cow says moo";
            let replaced = text.replace("moo", "woof");
            assert_eq!(replaced, "Cow says woof");
        }

        #[test]
        fn test_accept_either_string_or_str() {
            assert_eq!(accept_either("foo"), "foofoo");
            assert_eq!(accept_either(&String::from("bar")), "foobar");
        }
    }
}

mod iterators {
    #[cfg(test)]
    mod tests {
        #[test]
        fn test_owned_iterators() {
            let v = vec![1, 2, 3, 4, 5];

            for e in v.into_iter() {
                // this consumes the vector
                print!("{} ", e);
            }
            println!();

            //println!("{:?}", v);
        }

        #[test]
        fn test_borrowed_iterators() {
            let v = vec![1, 2, 3, 4, 5];
            for e in v.iter() {
                // this borrows the vector
                print!("{} ", e);
            }
            println!();

            // so this now fine
            println!("{:?}", v);
        }

        #[test]
        fn test_mutably_borrowed_iterators() {
            let mut v = vec![1, 2, 3, 4, 5];
            for e in v.iter_mut() {
                *e += 100;
            }
            println!("{:?}", v);
        }

        #[test]
        fn test_manual_iterator() {
            let v = vec!['a', 'b', 'c', 'd', 'e'];
            let mut iter = v.iter();
            while let Some(e) = iter.next() {
                print!("{} ", e);
            }
            println!();
        }

        #[test]
        fn test_combinator_map() {
            let fizzbuzz = (0..100).map(|x| match x {
                x if x % 15 == 0 => String::from("FizzBuzz"),
                x if x % 3 == 0 => String::from("Fizz"),
                x if x % 5 == 0 => String::from("Buzz"),
                _ => format!("{}", x),
            });

            for item in fizzbuzz {
                print!("{} ", item);
            }
            println!();
        }

        #[test]
        fn test_combinator_filter() {
            let evens = (1..100).filter(|&e| e % 2 == 0);

            for e in evens {
                print!("{} ", e);
            }
            println!();
        }

        #[test]
        fn test_combinator_filter_map() {
            // note that iterators are lazy by default
            let evens_squared = (1..100).filter(|&x| x % 2 == 0).map(|x| x * x);
            for e in evens_squared {
                print!("{} ", e);
            }
            println!();
        }
    }
}

mod functions {
    use std::fmt::Debug;

    #[derive(Debug)]
    struct Person {
        pub name: String,
        pub age: u16,
    }

    fn prints_anything<T: Debug>(thing: T) {
        println!("{:?}", thing);
    }

    // this is the same as above
    fn prints_anything_as_well<T>(thing: T)
    where
        T: Debug,
    {
        println!("{:?}", thing);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_prints_anything() {
            prints_anything("Hello");
            prints_anything(100);
            prints_anything(Person {
                name: "Bob".to_string(),
                age: 42,
            });
        }
    }

    #[test]
    fn test_prints_anything_as_well() {
        prints_anything_as_well("Hello");
        prints_anything_as_well(100);
        prints_anything_as_well(Person {
            name: "Bob".to_string(),
            age: 42,
        });
    }
}

mod stack_and_heap {
    #[cfg(test)]
    mod tests {
        #[test]
        fn test_stack_allocation() {
            #[derive(Debug, PartialEq)]
            struct Point {
                x: i32,
                y: i32,
            }

            // this is allocated on the stack
            let point = Point { x: 1, y: -32 };
            assert_eq!(point, Point { x: 1, y: -32 });
        }

        #[test]
        fn test_heap_allocation() {
            #[derive(Debug, PartialEq)]
            struct Point {
                x: i32,
                y: i32,
            }

            let point = Box::new(Point { x: 1, y: -32 });
            assert_eq!(*point, Point { x: 1, y: -32 });
        }
    }

    #[test]
    fn test_borrowing_box_values() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u16,
        }

        let boxed_bob = Box::new(Person {
            name: "Bob".to_string(),
            age: 42,
        });

        fn print_person(p: &Person) {
            println!("Name = {:?}, age = {:?}", p.name, p.age);
        }

        print_person(&boxed_bob);
    }
}

mod generics {
    #[derive(Debug, PartialEq)]
    struct Point<Precision> {
        x: Precision,
        y: Precision,
    }

    #[derive(Debug, PartialEq)]
    enum Either<L, R> {
        Left(L),
        Right(R),
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_precision() {
            let point = Point { x: 1, y: -32 };
            let another_point: Point<i32> = Point { x: 1, y: -32 };
            assert_eq!(point, another_point);
        }

        #[test]
        fn test_either() {
            let mut val = Either::Left::<i32, f64>(100);
            println!("val  = {:?}", val);
            val = Either::Right::<i32, f64>(1.2345);
            println!("val  = {:?}", val);
        }
    }
}
