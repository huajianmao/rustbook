pub fn run() {
  println!("RefCell::interior");

  let _a = 5;
  // let b = &mut a; // <- Not compile!

  // let mut c = 10;
  // let d = &c;
  // *d = 20; // <- Not compile!
}
