use std::thread;

pub fn run() {
  println!("Chapter 16::Creating Threads::move closures");

  let v = vec![1, 2, 3];

  // move needed as thread may longlive beyond main
  let handle = thread::spawn(move || {
    println!("Here is a vector: {:?}", v);
  });

  handle.join().unwrap();
}
