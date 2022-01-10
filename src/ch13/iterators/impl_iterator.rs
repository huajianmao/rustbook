struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}

impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.count < 5 {
      self.count += 1;
      Some(self.count)
    } else {
      None
    }
  }
}

fn assert_if_not_equal() {
  let mut counter = Counter::new();
  assert_eq!(counter.next(), Some(1));
  assert_eq!(counter.next(), Some(2));
  assert_eq!(counter.next(), Some(3));
  assert_eq!(counter.next(), Some(4));
  assert_eq!(counter.next(), Some(5));
  println!("assert_if_not_equal runs OK!");
}

fn using_other_iterator_trait_methods() {
  let v1 = vec![2, 3, 4, 5, 6];
  let v2 = vec![5, 6, 7, 8];
  let temp: Vec<_> = v1.iter().zip(v2.iter()).collect();
  println!("{:?}", temp);

  let sum: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();
  assert_eq!(18, sum);
  println!("using_other_iterator_trait_methods runs OK!");
}

pub fn run() {
  println!("Iterators::impl_iterators");
  assert_if_not_equal();
  using_other_iterator_trait_methods();
}
