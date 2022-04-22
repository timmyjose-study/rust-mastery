//! The std::cell module types.

// Cell operates on values.
mod cell {
    use std::cell::Cell;

    #[derive(Debug)]
    struct Person {
        name: String,
        age: Cell<u16>,
    }

    impl Person {
        pub fn new(name: &str, age: u16) -> Self {
            Person {
                name: name.into(),
                age: Cell::new(age),
            }
        }
    }

    // can mutate `age` even through an immutable reference
    fn birthday(p: &Person) {
        p.age.set(p.age.get() + 1);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_cell() {
            let bob = Person::new("Bob", 42);
            println!("[Cell] Bob is {:?}", bob);
            birthday(&bob);
            println!("[Cell] Bob is {:?}", bob);
        }
    }
}

// RefCell operates on references.
mod refcell {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    #[cfg(test)]
    mod tests {
        use std::{cell::Cell, marker::PhantomData, process::abort, ptr::NonNull};

        use super::*;

        #[test]
        fn test_mutating_data_inside_immutable_structures() {
            let shared_map = Rc::new(RefCell::new(HashMap::new()));
            {
                let mut map = shared_map.borrow_mut();
                map.insert("africa", 100);
                map.insert("asia", 200);
                map.insert("europe", 300);
                map.insert("americas", 400);
                map.insert("oceania", 500);
            }

            let total = shared_map.borrow().values().sum::<i32>();
            println!("Sum of all continents = {}", total);
        }

        #[test]
        fn test_impl_details_of_logically_immutable_methods() {
            struct Graph {
                edges: Vec<(i32, i32)>,
                span_tree_cache: RefCell<Option<Vec<(i32, i32)>>>,
            }

            impl Graph {
                fn mst(&self) -> Vec<(i32, i32)> {
                    self.span_tree_cache
                        .borrow_mut()
                        .get_or_insert_with(|| self.calc_span_tree())
                        .clone()
                }

                fn calc_span_tree(&self) -> Vec<(i32, i32)> {
                    vec![]
                }
            }

            let graph = Graph {
                edges: vec![(1, 2), (1, 3), (2, 4), (4, 5)],
                span_tree_cache: RefCell::new(None),
            };
            println!("{:?}", graph.mst());
        }

        #[test]
        fn test_mutating_impls_of_clone() {
            struct ArCee<T: ?Sized> {
                ptr: NonNull<ArCeeBox<T>>,
                phantom: PhantomData<ArCeeBox<T>>,
            }

            struct ArCeeBox<T: ?Sized> {
                strong: Cell<usize>,
                refcount: Cell<usize>,
                value: T,
            }

            impl<T: ?Sized> Clone for ArCee<T> {
                fn clone(&self) -> Self {
                    self.inc_strong();
                    ArCee {
                        ptr: self.ptr,
                        phantom: PhantomData,
                    }
                }
            }

            trait ArCeeBoxPtr<T: ?Sized> {
                fn inner(&self) -> &ArCeeBox<T>;

                fn strong(&self) -> usize {
                    self.inner().strong.get()
                }

                fn inc_strong(&self) {
                    self.inner()
                        .strong
                        .set(self.strong().checked_add(1).unwrap_or_else(|| abort()));
                }
            }

            impl<T: ?Sized> ArCeeBoxPtr<T> for ArCee<T> {
                fn inner(&self) -> &ArCeeBox<T> {
                    unsafe { self.ptr.as_ref() }
                }
            }
        }
    }
}
