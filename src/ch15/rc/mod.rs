mod rc_count;
mod simple;

pub fn run() {
  println!("Rc<T>");

  simple::run();
  rc_count::run();
}
