pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

pub struct Node {
    val: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, val: i32) {
        let new_node = Box::new(Node {
            val: val,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.val
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        // mem::replace here is making the current link empty, and returns the link in its previous state
        // before it was replaced. This value going out of scope frees the Box, freeing the Box frees the Node,
        // but in the while loop, it's the current links next node which gets emptied. Thus, freeing the boxed node
        // does not recursively drop nodes further in the list.

        // We do this iteratively because the default behavior is not tail-recursive.
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
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
