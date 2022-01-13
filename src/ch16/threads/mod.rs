mod moves;
mod spawn;

pub fn run() {
  println!("Chapter 16::Creating Threads");

  spawn::run();
  moves::run();
}
