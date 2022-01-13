mod encapsulation;
mod state;
mod trait_object;

pub fn run() {
  println!("Chapter 17");

  encapsulation::run();
  trait_object::run();
  state::run();
}
