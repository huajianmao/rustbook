pub fn run() {
  println!("[Dangling]");
  let x = 5;
  let r = &x;
  {
    let _x = 5;
    // r = &_x; // borrow checker will not allow this
  }

  println!("r: {}", r);
}
