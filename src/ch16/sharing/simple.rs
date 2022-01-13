use std::sync::Mutex;

pub fn run() {
  println!("Chapter 16::Sharing State::Mutex");

  let m = Mutex::new(5);

  {
    let mut num = m.lock().unwrap();
    *num = 6;
  }

  println!("m = {:?}", m);
}
