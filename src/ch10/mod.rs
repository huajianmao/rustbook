mod generics;
mod lifetimes;
mod traits;

pub fn run() {
  generics::run();
  traits::run();
  lifetimes::run();
}
