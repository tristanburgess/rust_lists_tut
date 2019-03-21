use std::mem;

pub struct List {
    head: Link,
}

pub enum Link {
    Empty,
    Elem(Box<Node>),
}

pub struct Node {
    val: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, val: i32) {
        let new_node = Box::new(Node {
            val: val,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::Elem(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::Elem(boxed_node) => {
                let node = *boxed_node;
                self.head = node.next;
                Some(node.val)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        // mem::replace here is making the current link empty, and returns the link in its previous state
        // before it was replaced. This value going out of scope frees the Box, freeing the Box frees the Node,
        // but in the while loop, it's the current links next node which gets emptied. Thus, freeing the boxed node
        // does not recursively drop nodes further in the list.

        // We do this iteratively because the default behavior is not tail-recursive.
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::Elem(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
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
    }
}
