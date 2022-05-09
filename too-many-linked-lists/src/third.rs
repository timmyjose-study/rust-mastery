use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node::new(elem, self.head.clone()))),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(node) = cur_link {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                cur_link = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T, next: Link<T>) -> Self {
        Node { elem, next }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn prepend() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list1 = list.prepend(1).prepend(2).prepend(3);
        println!("third:prepend:list1 = {:?}", list1);
        assert_eq!(list1.head(), Some(&3));

        let list2 = list1.tail();
        println!("third:prepend:list1 = {:?}", list2);
        assert_eq!(list2.head(), Some(&2));

        assert_eq!(list1.head(), Some(&3));

        let list = list2.tail().tail().tail();
        println!("third:prepend:list = {:?}", list);
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
