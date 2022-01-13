use std::sync::mpsc;
use std::thread;

pub fn run() {
  println!("Chapter 16::Message Passing::Single Value");

  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let msg = String::from("hi");
    tx.send(msg).unwrap();
  });

  let received = rx.recv().unwrap();
  println!("Received data is: {}", received);
}
