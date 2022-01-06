pub fn run() {
  println!("[Generics]");

  let s1 = String::from("abcd");
  let s2 = String::from("xyz");

  let result = longest(s1.as_str(), s2.as_str());
  println!("The longest string is {}", result);

  let _fail;
  {
    let s3 = String::from("fail");
    _fail = longest(s1.as_str(), s3.as_str());
    println!("The longest string is {}", _fail);
  }

  // // This will not compile because the type of s3 is no longer valid
  // println!("The longest string is {}", _fail);
}

// // Will fail, as lifetime is not specified
// fn longest(s1: &str, s2: &str) -> &str {
//   if s1.len() > s2.len() {
//     s1
//   } else {
//     s2
//   }
// }

// The result will have a lifetime equal to the shorter lifetime of the two strings
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
  if s1.len() > s2.len() {
    s1
  } else {
    s2
  }
}

// fn can_not_return_reference_to_local_variable() -> &str {
//   let s1 = String::from("abcd");
//   s1.as_str()
// }
