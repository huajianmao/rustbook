#[derive(Debug)]
enum List {
  Cons(i32, Box<List>), // <---- Attention
  Nil,
}

use List::{Cons, Nil};

pub fn run() {
  println!("Box::List");
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  println!("{:?}", list);
}
