use std;
use std::fmt;

// enum is from https://doc.rust-lang.org/book/ch15-01-box.html
#[derive(Debug)]
pub enum ConsList<T> {
    Cons(T, Box<ConsList<T>>),
    Nil,
}

/// Get the head of the ConsList
pub fn car<T>(cons: &ConsList<T>) -> Option<&T> {
    match cons {
        ConsList::Cons(car, _cdr) => Some(car),
        ConsList::Nil => None,
    }
}

/// Get the tail of the ConsList
pub fn cdr<T>(cons: &ConsList<T>) -> Option<&ConsList<T>> {
    match cons {
        ConsList::Cons(_car, cdr) => Some(cdr),
        ConsList::Nil => None,
    }
}

/// Recursively pretty-print the ConsList
///
/// # Arguments
///
/// * `cons` - The ConsList to print
/// * `indent` - The number of indentions to indent the list
///
impl<T> ConsList<T>
where
    T: fmt::Display,
{
    pub fn pretty_print(cons: &ConsList<T>, indent: usize) -> String {
        let indention_size: usize = 2; // number of spaces per indention

        match cons {
            ConsList::Cons(car, cdr) => format!(
                "\n{}Cons({}, {})",
                "  ".repeat(indent),
                car,                                              // print value in car
                Self::pretty_print(cdr, indent + indention_size) // increase indention by 1 indent and print cdr
            ),
            ConsList::Nil => String::from("Nil"),
        }
    }
}

/// Format ConsList in LISP dotted pair notation
impl<T> fmt::Display for ConsList<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConsList::Cons(car, cdr) => write!(f, "( {} . {} )", car, cdr),
            ConsList::Nil => write!(f, "Nil"),
        }
    }
}
