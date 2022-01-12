use std::rc::Rc;

enum List {
  Cons(i32, Rc<List>),
  Nil,
}

use List::{Cons, Nil};

pub fn run() {
  println!("Rc<T>::Rc_count");

  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  println!("Count after creating a = {}", Rc::strong_count(&a));

  let _b = Cons(3, Rc::clone(&a));
  println!("Count after creating b = {}", Rc::strong_count(&a));

  {
    let _c = Cons(4, Rc::clone(&a));
    println!("Count after creating c = {}", Rc::strong_count(&a));
  }

  println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
}
