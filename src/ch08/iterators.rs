pub fn run() {
  let mut v = vec![1, 2, 3, 4, 5];
  for i in &v {
    println!("{}", i);
  }
  // let _ = &v.iter().for_each(|i| println!("{}", i));

  for i in &mut v {
    *i += 50;
  }
  let _ = &v.iter().for_each(|i| println!("{}", i));
}
