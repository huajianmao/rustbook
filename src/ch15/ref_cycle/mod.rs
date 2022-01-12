mod count;
mod cycle;
mod weak;

pub fn run() {
  println!("Reference_Cycles");

  cycle::run();
  weak::run();
  count::run();
}
