use std::fmt::Display;

struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x > self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}

pub fn run() {
  let p1 = Pair::new(1, 2);
  p1.cmp_display();

  let _p2 = Pair::new([1, 2], [2, 3]);
  // _p2.cmp_display(); will not compile
}
