struct ImportantExcerpt<'a> {
  part: &'a str,
}

pub fn run() {
  println!("[Structs]");

  let novel = String::from("Call me Huajian. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");

  let i = ImportantExcerpt {
    part: first_sentence,
  };
  println!("The first sentence is: {}", i.part);
}
