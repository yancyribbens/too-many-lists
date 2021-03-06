
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

    // push mutates the list, so, a mutable reference
    // to self is passed.
    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            //next: self.head
            next: mem::replace(&mut self.head, Link::Empty)
        };

        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        let a = match &mut self.head {
            Link::Empty => None,

            Link::More(box y) => {
                let old_val = mem::replace(y, Node { elem: 0, next: Link::Empty } );
                self.head = old_val.next;
                Some(old_val.elem)
            }
        };
 
        a
    }
}

mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);
        
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
