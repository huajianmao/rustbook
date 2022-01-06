use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
  T: Display,
{
  println!("Announcement! {}", ann);

  if x.len() > y.len() {
    x
  } else {
    y
  }
}

pub fn run() {
  let x = "hello";
  let y = "world";
  let ann = "this is an announcement";
  let result = longest_with_an_announcement(x, y, ann);
  println!("The longest string is {}", result);
}
