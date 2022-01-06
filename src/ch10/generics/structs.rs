#[derive(Debug)]
struct Point<T, U> {
  x: T,
  y: U,
}

impl<T, U> Point<T, U> {
  fn x(&self) -> &T {
    &self.x
  }
}

impl Point<i32, i32> {
  fn y(&self) -> i32 {
    self.y
  }
}

impl<T, U> Point<T, U> {
  fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    Point {
      x: self.x,
      y: other.y,
    }
  }
}

pub fn run() {
  let p1 = Point { x: 5, y: 10 };
  p1.y();

  println!("{:?}", (p1.x(), p1.y));

  let _p2 = Point { x: 5.0, y: 10.0 };
  // _p2.y(); will not usable for Point<f64, f64>

  let _p3 = Point { x: 5, y: 10.0 };

  let p4 = p1.mixup(_p2);
  println!("{:?}", p4);
}
