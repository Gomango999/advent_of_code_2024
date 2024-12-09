//! My attempt at writing a doubly-linked list library. It offers a couple
//! functions that the standard library collection does not offer: namely
//! insert_after and insert_before
//! Todo:
//! - [ ] Add ability to declare from an existing array / vector
//! - [ ] Add iterators
//! - [ ] Add common traits (e.g. Display)
//! - [ ] Make sure it works when immutable and mutable
//! - [ ] Do I expose the Node type directly. Try to implement the Cursor?
//! - [ ] Look at how the standard library implements
//! - [ ] Other features: clear, split

use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct LinkedList<T> {
    list: Rc<RefCell<LinkedListImpl<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        let list = Rc::new(RefCell::new(LinkedListImpl::new()));
        LinkedList { list }
    }
}

struct LinkedListImpl<T> {
    len: usize,
    front: Option<Rc<Node<T>>>,
    back: Option<Weak<Node<T>>>,
}

impl<T> LinkedListImpl<T> {
    fn new() -> Self {
        LinkedListImpl {
            len: 0,
            front: None,
            back: None,
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
    fn front(&self) -> Option<T> {
        todo!()
    }
    fn back(&self) -> Option<T> {
        todo!()
    }
    fn mut_cursor_front(self) -> MutCursor<T> {
        MutCursor::new()
    }
    fn push_front(&mut self) {
        todo!()
    }
    fn push_back(&mut self) {
        todo!()
    }
    fn pop_front(&mut self) -> Option<T> {
        todo!()
    }
    fn pop_back(&mut self) -> Option<T> {
        todo!()
    }
}

struct MutCursor<'a, T>
where
    T: 'a,
{
    list: Weak<RefCell<LinkedListImpl<T>>>,
    ptr: Option<Weak<Node<T>>>,
}

impl<'a, T> MutCursor<'a, T> {
    fn new(list: &'a Rc<RefCell<LinkedListImpl<T>>>) -> Self {
        MutCursor {
            list: Rc::downgrade(&list),
            ptr: list.borrow().front.as_ref().map(|ptr| Rc::downgrade(ptr)),
        }
    }

    /// Returns whether the underlying element it is pointing to is still in
    /// the LinkedList.
    fn is_valid(&self) -> bool {
        self.ptr.is_some()
    }

    fn peek(&self) -> Option<&T> {
        let node_rc = self.ptr.as_ref().and_then(|weak_node| weak_node.upgrade());
        node_rc.map(|node_rc| &node_rc.value)

        // self: &MutCursor<'a, T> - end of function
        // .ptr: Option<Weak<Node<T>> - end of function
        // .as_ref(): Option<&Weak<Node<T>> - end of ptr
        // ?: &Weak<Node<T>>a - end of ptr
        // .upgrade(): Option<Rc<Node<T>>> - end of function
        // .map(...):
        //      node: Rc<Node<T>> - end of map
        //      .as_ref(): &Node<T> - end of map
        //      .peek(): &T - end of map
    }
}

struct Node<T> {
    value: T,
    next: Option<Rc<Node<T>>>,
    prev: Option<Weak<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            next: None,
            prev: None,
        }
    }

    fn peek(&self) -> &T {
        &self.value
    }

    fn peek_next(&self) -> Option<&T> {
        todo!()
    }

    fn peek_prev(&self) -> Option<&T> {
        todo!()
    }

    /// Returns a pointer to the next node.
    ///
    /// This returns `None` if it is already at the end.
    fn next(&self) -> Option<Rc<Node<T>>> {
        self.next.as_ref().map(|ptr| Rc::clone(&ptr))
    }

    /// Returns a pointer to the previous node.
    ///
    /// This returns `None` if it is already at the beginning.
    fn prev(&self) -> Option<Rc<Node<T>>> {
        self.prev.as_ref().map(|ptr| ptr.upgrade()).flatten()
    }

    fn insert_after(&mut self, value: T) {
        let a = self;
        let b = Node::new(value);
        let c = a.next();
        if c.is_none() {
        } else {
        }
    }

    fn insert_before(&mut self, value: T) {
        todo!()
    }
}
