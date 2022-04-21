//! Async Rust.

mod non_async {
    use std::fs::File;
    use std::io::{self, Read};
    use std::path::Path;

    // non-async version
    fn read_from_disk<P: AsRef<Path> + ?Sized>(p: &P) -> io::Result<String> {
        let mut reader = File::open(p.as_ref())?;

        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;

        Ok(contents)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_non_async_read_from_disk() {
            println!(
                "{}",
                match read_from_disk("src/async.rs") {
                    Err(e) => e.to_string(),
                    Ok(contents) => contents,
                }
            );
        }
    }
}

mod async_hello_world {
    use async_std::fs::File;
    use async_std::io::{self, ReadExt};
    use async_std::path::Path;

    async fn read_from_disk<P: AsRef<Path> + ?Sized>(p: &P) -> io::Result<String> {
        let mut reader = File::open(p.as_ref()).await?;

        let mut contents = String::new();
        reader.read_to_string(&mut contents).await?;

        Ok(contents)
    }

    async fn say_hello() {
        println!("Hello, world!");
    }

    async fn read_from_stdin() -> io::Result<String> {
        let stdin = io::stdin();
        let mut line = String::new();
        stdin.read_line(&mut line).await?;
        Ok(line)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use async_std::task;

        #[test]
        fn test_async_read_from_disk() {
            let read_future = read_from_disk("src/async.rs");

            let result = task::block_on(async {
                let task = task::spawn(read_future);
                task.await
            });

            println!(
                "{}",
                match result {
                    Err(e) => e.to_string(),
                    Ok(contents) => contents,
                }
            );
        }

        #[test]
        fn test_say_hello() {
            task::block_on(async {
                let task = task::spawn(say_hello());
                task.await
            });
        }

        #[test]
        fn test_combining_futures() {
            use async_std::io;
            use std::time::Duration;

            let read_future = read_from_disk("src/async.rs");
            let timeout = Duration::from_millis(1000);
            let timeout_read = io::timeout(timeout, read_future);

            let result = task::block_on(async {
                let task = task::spawn(timeout_read);
                task.await
            });

            println!("{:?}", result);
        }

        #[test]
        fn test_async_read_from_stdin() {
            let stdin_future = read_from_stdin();
            let result = task::block_on(async {
                let task = task::spawn(stdin_future);
                task.await
            });

            if let Ok(line) = result {
                println!("{}", line);
            }
        }
    }
}
