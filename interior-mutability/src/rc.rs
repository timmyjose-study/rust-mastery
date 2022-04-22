//! The std::rc module types.

mod rc {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[cfg(test)]
    mod tests {
        use std::rc::Weak;

        use super::*;

        #[test]
        fn test_one_way_owner_to_owned() {
            #[derive(Debug)]
            struct Owner {
                name: String,
            }

            #[derive(Debug)]
            struct Gadget {
                id: i32,
                owner: Rc<Owner>,
            }

            let bob = Rc::new(Owner { name: "Bob".into() });

            let gadget1 = Gadget {
                id: 1,
                owner: Rc::clone(&bob),
            };

            let gadget2 = Gadget {
                id: 2,
                owner: Rc::clone(&bob),
            };

            drop(bob);

            println!("{:?}'s owner is {:?}", gadget1, gadget1.owner);
            println!("{:?}'s owner is {:?}", gadget2, gadget2.owner);
        }

        #[test]
        fn test_two_way_traversal_between_owner_and_owned() {
            #[derive(Debug)]
            struct Owner {
                name: String,
                gadgets: RefCell<Vec<Weak<Gadget>>>,
            }

            #[derive(Debug)]
            struct Gadget {
                id: i32,
                owner: Rc<Owner>,
            }

            let bob = Rc::new(Owner {
                name: "Bob".into(),
                gadgets: RefCell::new(Vec::new()),
            });

            let gadget1 = Rc::new(Gadget {
                id: 1,
                owner: Rc::clone(&bob),
            });

            let gadget2 = Rc::new(Gadget {
                id: 2,
                owner: Rc::clone(&bob),
            });

            {
                let mut gadgets = bob.gadgets.borrow_mut();
                gadgets.push(Rc::downgrade(&gadget1));
                gadgets.push(Rc::downgrade(&gadget2));
            }

            for gadget in bob.gadgets.borrow().iter() {
                if let Some(gadget) = gadget.upgrade() {
                    println!("{:?}", gadget);
                }
            }
        }
    }
}
