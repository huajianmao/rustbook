use std::rc::Rc;

enum List {
  Cons(i32, Rc<List>),
  Nil,
}

use List::{Cons, Nil};

pub fn run() {
  println!("Rc<T>::Simple");

  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  let _b = Cons(3, Rc::clone(&a));
  let _c = Cons(4, Rc::clone(&a)); // <-- Multiple owner
}
