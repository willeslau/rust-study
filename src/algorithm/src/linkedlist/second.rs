use std::ops::Deref;

pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        self.head = Some(Box::new(Node { value, next: self.head.take() }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.value
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.value
        })
    }
}

/// The iterator for List<T>
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|v| {
            self.next = v.next.as_ref().map(|a| a.deref());
            &v.value
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::linkedlist::second::List;

    #[test]
    fn simple_list_works() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.pop().unwrap(), 3);

        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.pop().unwrap(), 2);

        assert_eq!(list.peek(), Some(&1));
        let mut i = list.peek_mut();
        i = i.map(|mut n| {
            *n += 1;
            n
        });
        assert_eq!(list.pop().unwrap(), 2);

        assert_eq!(list.peek(), None);
        assert_eq!(list.pop().is_none(), true);
    }

    #[test]
    fn test_iterator_related() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        {
            let f = iter.next();
            assert_eq!(f, Some(&3));
            drop(f);
        }
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}