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
        let f = File::open("src/basic.rs")?;
        let x = "123abc".parse::<i32>()?;

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
        x: i32,
        y: i32,
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
    #[cfg(test)]
    mod tests {
        use super::*;
    }
}
