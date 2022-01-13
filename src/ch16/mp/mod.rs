mod multi_senders;
mod multi_values;
mod single;

pub fn run() {
  println!("Chapter 16::Message Passing");

  single::run();
  multi_values::run();
  multi_senders::run();
}
