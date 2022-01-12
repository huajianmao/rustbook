mod deref;
mod rc;
mod ref_cycle;
mod refcell;
mod smart_pointer;

pub fn run() {
  println!("Hello Chpater 15");

  smart_pointer::run();
  deref::run();
  rc::run();
  refcell::run();
  ref_cycle::run();
}
