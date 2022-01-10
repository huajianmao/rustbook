mod example;
mod filters;
mod how_it_works;
mod impl_iterator;

pub fn run() {
  println!("Iterators::");
  example::run();
  how_it_works::run();
  filters::run();
  impl_iterator::run();
}
