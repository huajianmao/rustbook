pub fn run() {
  println!("[Static Lifetimes]");

  let _s: &'static str = "I have a static lifetime";
}
