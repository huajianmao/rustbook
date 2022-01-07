use unicode_segmentation::UnicodeSegmentation;

pub fn run() {
  // Strings are stored as a collection of UTF-8 encoded bytes
  let s1 = String::new();
  let s2 = "initial contents";
  let _s3 = s2.to_string();
  let _s4 = String::from("Hello world!");

  let mut s = String::new();
  s.push_str("foo");
  s.push_str("bar");
  s.push('!');
  println!("{}", s);

  let _s3 = format!("{}{}", s1, s2);

  let _s3 = s1 + s2;
  // println!("{}", s1); // This will not compile

  let hello = String::from("你好！a̐é");
  hello.bytes().for_each(|b| println!("{}", b));
  hello.chars().for_each(|c| println!("{}", c));
  hello.graphemes(true).for_each(|g| println!("{}", g));
}
