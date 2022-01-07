mod enum_vectors;
mod hashmap;
mod iterators;
mod strings;
mod vectors;

pub fn run() {
  println!("Chapter 8");

  vectors::run();
  iterators::run();
  enum_vectors::run();
  strings::run();
  hashmap::run();
}
