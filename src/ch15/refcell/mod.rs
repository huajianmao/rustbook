mod interior;
mod mock;
mod rc_with_refcell;

pub fn run() {
  println!("RefCell");

  interior::run();
  mock::run();
  rc_with_refcell::run();
}
