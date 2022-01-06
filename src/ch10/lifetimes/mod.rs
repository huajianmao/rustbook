mod all;
mod dangling;
mod generics;
mod rules;
mod static_lifetime;
mod structs;

pub fn run() {
  println!("[Lifetimes]");

  dangling::run();
  generics::run();
  structs::run();
  rules::run();
  static_lifetime::run();
  all::run();
}
