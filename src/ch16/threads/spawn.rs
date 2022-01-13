use std::thread;
use std::time::Duration;

pub fn run() {
  println!("Chapter 16::Creating Threads::spawn");

  // One thread spawned!
  let handler = thread::spawn(|| {
    for i in 0..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 0..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }

  handler.join().unwrap();
}
