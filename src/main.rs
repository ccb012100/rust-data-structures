use core::panic;
use std::fmt;

use rust_data_structures::ConsList::{Cons, Nil};
use rust_data_structures::{lists, ConsList};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("*** list:\n");
    parse_cons_list(list);
}

fn parse_cons_list<T>(cons: ConsList<T>)
where
    T: fmt::Display,
{
    println!("{}", cons);

    let mut head = lists::car(&cons);
    let mut cons = lists::cdr(&cons);

    while let Some(h) = head {
        println!("head => {}", h);
        match cons {
            Some(t) => {
                println!("tail => {}\n", t);
                head = lists::car(t);
                cons = lists::cdr(t);
            }
            None => panic!("This should never come back as None."),
        }
    }
}
