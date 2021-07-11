struct Node {
    value: i32,
    next: Link,
}

pub struct List {
    head: Link,
}

pub enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, value: i32) {
        let node = Node {
            value,
            next: std::mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        // NOTE: one way of writing is the below, it is essentially `pop`:
        // loop {
        //     match std::mem::replace(&mut self.head, Link::Empty) {
        //         Link::Empty => break,
        //         Link::More(node) => {
        //             self.head = node.next;
        //         }
        //     }
        // }
        // NOTE: but there is a major drop back, we are moving node.next into self.head
        // NOTE: if the item we are moving is big, then the efficiency is low.
        let mut link = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut node) = link {
            link = std::mem::replace(&mut node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::linkedlist::first::List;

    #[test]
    fn simple_list_works() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop().unwrap(), 3);
        assert_eq!(list.pop().unwrap(), 2);
        assert_eq!(list.pop().unwrap(), 1);
        assert_eq!(list.pop().is_none(), true);

    }
}