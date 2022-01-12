mod deref;
mod drop;
mod simple;

pub fn run() {
  println!("Smart Pointer::DeRef");
  simple::run();
  deref::run();
  drop::run();
}
