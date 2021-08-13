/*
 * Linked List structure based on
 * https://rust-unofficial.github.io/too-many-lists/first-layout.html
*/
use std::cell::RefCell;
use std::mem;

#[derive(Debug)]
pub struct LinkedList {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Nil,
    Next(Box<Node>),
}

#[derive(Debug)]
pub struct Node {
    data: i32,
    next: RefCell<Link>,
}

impl LinkedList {
    pub fn create(data: i32) -> Self {
        Self {
            head: Link::Next(Box::new(Node::new(data))),
        }
    }

    pub fn new() -> Self {
        Self { head: Link::Nil }
    }

    pub fn traverse(list: &LinkedList) {
        match &list.head {
            Link::Nil => println!("Empty list!"),
            Link::Next(next) => Node::traverse(&*next),
        }
    }

    pub fn get_head(&mut self) -> Option<&mut Node> {
        match self.head {
            Link::Nil => None,
            Link::Next(ref mut next) => Some(next),
        }
    }
}

impl Default for LinkedList {
    fn default() -> Self {
        Self::new()
    }
}

impl Node {
    pub fn new(data: i32) -> Self {
        Self {
            data,
            next: RefCell::new(Link::Nil),
        }
    }

    pub fn traverse(node: &Node) {
        println!("Node: {}", node.data);

        match &*node.next.borrow() {
            Link::Nil => println!("End of list."),
            Link::Next(n) => Self::traverse(&*n),
        }
    }

    pub fn append_after(&mut self, mut node: Node) {
        // I'm sure I'm doing this wrong, but it works...
        mem::swap(&mut self.next, &mut node.next); // move self.next into tmp
        self.next = RefCell::new(Link::Next(Box::new(node))); // move node to self.next
    }
}
