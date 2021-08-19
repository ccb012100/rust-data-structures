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

    // traverse the list
    pub fn traverse(list: &LinkedList) {
        match &list.head {
            Link::Nil => println!("Empty list!"),
            Link::Next(next) => Node::traverse(&*next),
        }
    }

    // Get the head node of the list
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

    // traverse from `node` to end of List
    pub fn traverse(node: &Node) {
        println!("Node: {}", node.data);

        match &*node.next.borrow() {
            Link::Nil => println!("End of list."),
            Link::Next(n) => Self::traverse(&*n),
        }
    }

    // append new Node after `self`
    pub fn append_after(&mut self, mut node: Node) {
        // move self.Next value into node.Next
        node.next = mem::replace(&mut self.next, RefCell::new(Link::Nil));
        // set self.next to node
        self.next = RefCell::new(Link::Next(Box::new(node)));
    }

    // append new Node before `self`
    pub fn append_before(&mut self, node: Node) {
        // move self into tmp
        let tmp = mem::replace(self, Node::new(0));
        // move new node into self
        *self = node;
        // set node.next to tmp
        self.next = RefCell::new(Link::Next(Box::new(tmp)));
    }
}
