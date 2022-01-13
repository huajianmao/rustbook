mod encoding;
mod post;

use post::Post;

pub fn run() {
  println!("Chapter 17::State Design Pattern");

  let mut post = Post::new();

  post.add_text("I ate a salad for lunch today!");
  assert_eq!("", post.content());

  post.request_review();
  assert_eq!("", post.content());

  post.approve();
  assert_eq!("I ate a salad for lunch today!", post.content());

  println!("State Design Pattern runs OK!");

  encoding::run();
}
