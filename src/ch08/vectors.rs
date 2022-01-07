pub fn run() {
  let _a = [1, 2, 3];
  let mut v: Vec<i32> = Vec::new();
  v.push(1);
  v.push(2);
  v.push(3);
  {
    let _v2 = vec![1, 2, 3];
  }

  let v = vec![1, 2, 3, 4, 5];
  let third = &v[2]; // <---- immutable reference
                     // v.push(6); // This will not compile
  println!("The third element is {}", third);
  // let _ = &v[20]; // This will panic

  match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
  }
}
