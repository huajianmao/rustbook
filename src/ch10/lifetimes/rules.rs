pub fn run() {
  println!("[Rules]");

  println!("Input lifetime -> Output lifetime");

  // 1. Each parameter that is a reference gets its own lifetime parameter.

  // 2. If there is exactly one input lifetime parameter,
  //    that lifetime is assigned to all output lifetime parameters.

  // 3. If there are multiple input lifetime parameters,
  //    but one of them is &self or &mut self,
  //    then the lifetime of self is assigned to all output lifetime parameters.

  // 4. otherwise, we need to specify an explicit output lifetime parameter.

  let novel = String::from("Call me Huajian. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");

  let i = ImportantExcerpt {
    part: first_sentence,
  };
  println!(
    "The first sentence is: {}",
    i.return_part("Some annoucement")
  );
}

struct ImportantExcerpt<'a> {
  part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
  // fn return_part(&'a self, announcement: &str) -> &'a str {
  fn return_part(&self, announcement: &str) -> &str {
    println!("Attention please: {}", announcement);
    self.part
  }
}
