use rust_data_structures::lists::conslist::ConsList;
use rust_data_structures::lists::linkedlist::{self, LinkedList, Node};

fn main() {
    // TODO: move these to tests
    linked_list();
}

fn cons_list() {
    let cons = ConsList::Cons(
        1,
        Box::new(ConsList::Cons(
            2,
            Box::new(ConsList::Cons(3, Box::new(ConsList::Nil))),
        )),
    );

    println!("*** CONSLIST:\n");
    ConsList::parse_cons_list(cons);
}

fn linked_list() {
    let linked = LinkedList::new();

    println!("*** EMPTY LINKED LIST:\n\n{:#?}", linked);
    LinkedList::traverse(&linked);

    let mut linked = LinkedList::create(100);
    let head: &mut linkedlist::Node = linked.get_head().unwrap();
    let node = Node::new(300);
    head.append_after(node);
    let node = Node::new(200);
    head.append_after(node);
    head.append_after(Node::new(300));

    println!("List:\n\n{:#?}", linked);
    LinkedList::traverse(&linked);
}
