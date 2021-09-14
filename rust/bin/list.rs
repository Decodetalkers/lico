enum List {
    Cons(u32, Box<List>),
    Nil,
}
use List::*;
impl List {
    fn new() -> Self {
        Nil
    }

    fn prepend(self, elem: u32) -> Self {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{},{}", head, tail.stringify())
            }
            Nil => "Nil".to_string(),
        }
    }
}
fn main() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length:{}", list.len());
    println!("{}", list.stringify());
}
