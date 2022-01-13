use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
  println!("Chapter 16::Message Passing::Multiple Senders");

  let (tx, rx) = mpsc::channel();
  let tx2 = tx.clone();

  thread::spawn(move || {
    let values = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for value in values {
      tx.send(value).unwrap();
      thread::sleep(Duration::from_millis(100));
    }
  });

  thread::spawn(move || {
    let values = vec![
      String::from("more"),
      String::from("message"),
      String::from("for"),
      String::from("you"),
    ];

    for value in values {
      tx2.send(value).unwrap();
      thread::sleep(Duration::from_millis(100));
    }
  });

  for received in rx {
    println!("Received data is: {:?}", received);
  }
}
