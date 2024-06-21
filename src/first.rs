use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem:i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_push() {
        let mut list = List::new();
        list.push(4);

        assert!( matches!( list.head, Link::More(n) if n.elem == 4 ) );

    }
}

