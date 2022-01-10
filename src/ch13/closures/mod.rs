mod cache;
mod env;
mod optimization;
mod original;

pub fn run() {
  println!("Hello Closures in Chpater 13");

  original::run();
  optimization::run();
  cache::run();
  env::run();
}
