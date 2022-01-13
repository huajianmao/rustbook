mod mp;
mod sharing;
mod threads;

pub fn run() {
  println!("Chapter 16");

  threads::run();
  mp::run();
  sharing::run();
}
