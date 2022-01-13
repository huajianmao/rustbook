mod multi_threads;
mod simple;

pub fn run() {
  println!("Chapter 16::Sharing State");

  simple::run();
  multi_threads::run();
}
