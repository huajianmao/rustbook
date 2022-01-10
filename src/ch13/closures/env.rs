pub fn run() {
  println!("Closures::env");

  // let x = 4;

  // let equal_to_x = |z| z == x;

  // // fn equal_to_x(z: i32) -> bool {
  // //   x == z // <---- can't capture dynamic environment in a fn item use the `|| { ... }` closure form instead
  // // }

  // let y = 4;
  // assert!(equal_to_x(y));

  // Three ways
  let x = vec![1, 2, 3];

  let equal_to_x = |z| z == x;
  let y = vec![1, 2, 3];
  assert!(equal_to_x(y));

  // let equal_to_x = move |z| z == x;
  // println!("Can't use x here: {:?}", x);
  // let y = vec![1, 2, 3];
  // assert!(equal_to_x(y));
}
