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
}

#[cfg(test)]
mod tests {
    use crate::linkedlist::simple::List;

    #[test]
    fn test() {
        let list = List::new();
    }
}