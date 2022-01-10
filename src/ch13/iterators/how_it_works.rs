pub trait Iterator {
  type Item; // <---- Attention
  fn next(&mut self) -> Option<Self::Item>; // <---- Attention
}

fn iterator_demonstration() {
  let v1 = vec![1, 2, 3];

  let mut v1_iter = v1.iter();
  assert_eq!(v1_iter.next(), Some(&1));
  assert_eq!(v1_iter.next(), Some(&2));
  assert_eq!(v1_iter.next(), Some(&3));
  assert_eq!(v1_iter.next(), None);

  let mut v1_iter = v1.into_iter();
  assert_eq!(v1_iter.next(), Some(1));
  assert_eq!(v1_iter.next(), Some(2));
  assert_eq!(v1_iter.next(), Some(3));
  assert_eq!(v1_iter.next(), None);

  println!("iterator_demonstration runs ok!");
}

fn iterator_sum() {
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();
  let total: i32 = v1_iter.sum();
  assert_eq!(total, 6);

  println!("iterator_sum runs ok!");
}

fn map_on_iterator() {
  let v1 = vec![1, 2, 3];
  let v2: Vec<_> = v1.iter().map(|v| v + 1).collect();
  assert_eq!(v2, vec![2, 3, 4]);

  println!("map_on_iterator runs ok!");
}

pub fn run() {
  println!("Iterators::how_it_works");

  iterator_demonstration();
  iterator_sum();
  map_on_iterator();
}
