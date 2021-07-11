struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

/// This struct is using ownership, then it will not have all those borrow nonsense.
/// But we need to clone this every time and it won't work most of the time.
pub struct ListOwnership {
    head: Option<Box<Node>>,
}

impl ListOwnership {
    pub fn new() -> Self {
        ListOwnership { head: None }
    }

    pub fn push(mut self, value: i32) {
        let mut node = Box::new(Node { value, next: None });
        node.next = self.head;
        self.head = Some(node);
    }

    pub fn pop(mut self) -> Option<i32> {
        match self.head {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

pub struct List {
    head: Option<Box<Node>>,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, value: i32) {
        // this code will not work: node.next = self.head;
        // because previously, self is borrowed, you cannot
        // move self.head to another variable
        match std::mem::replace(&mut self.head, None) {
            None => { self.head = Some(Box::new(Node{ value, next: None })); }
            Some(node) => {
                self.head = Some(Box::new(Node{ value, next: Option::from(node) }));
            }
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::linkedlist::my_list::{ListOwnership, List};

    #[test]
    fn my_list_works() {
        let list = ListOwnership::new();
        list.push(1);
        // list.push(2);
        // list.push(3);
        //
        // assert_eq!(list.pop().unwrap(), 3);
        // assert_eq!(list.pop().unwrap(), 2);
        // assert_eq!(list.pop().unwrap(), 1);
        // assert_eq!(list.pop().is_none(), true);
    }

    #[test]
    fn list_works() {
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