use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, val: T) {
        let mut new_tail = Box::new(Node {
            val: val,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }

        self.tail = raw_tail
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let head = *node;
            self.head = head.next;
            
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            head.val
        })
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
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        list.push(2);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), None);
    }
}
