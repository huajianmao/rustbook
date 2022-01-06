pub fn run() {
  a();
}

fn a() {
  b();
}

fn b() {
  c(22);
}

fn c(num: i32) {
  if num == 22 {
    panic!("Do not pass 22!");
  }
}
