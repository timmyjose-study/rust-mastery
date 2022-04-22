//! The std::sync module types.

mod arc {
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[cfg(test)]
    mod tests {
        use std::sync::Mutex;

        use super::*;

        #[test]
        fn test_sharing_immutable_data_between_threads() {
            let data = Arc::new(5);
            let mut handles = Vec::new();

            for _ in 0..5 {
                let data = Arc::clone(&data);

                handles.push(thread::spawn(move || {
                    thread::sleep(Duration::from_millis(250));
                    match *data {
                        5 => println!("five!"),
                        _ => println!("something else"),
                    }
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }
        }

        #[test]
        fn test_sharing_mutable_data_between_threads() {
            let data = Arc::new(Mutex::new(0));
            let mut handles = Vec::new();

            for _ in 0..10 {
                let data = Arc::clone(&data);

                handles.push(thread::spawn(move || {
                    let mut data = data.lock().unwrap();
                    *data += 1;
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }

            assert_eq!(*data.lock().unwrap(), 10);
        }
    }
}
