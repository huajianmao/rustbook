pub fn run() {
  println!("DeRef trait");

  let x = 5;
  let y = &x;
  assert_eq!(5, x);
  assert_eq!(5, *y); // DeRef here ...
                     // assert_eq!(5, y); // <-- This will not compile.

  let z = Box::new(x);
  assert_eq!(x, *z);

  println!("Simple runs OK");
}
