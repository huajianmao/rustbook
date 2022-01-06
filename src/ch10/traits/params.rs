use std::fmt::Debug;
use std::fmt::Display;

use super::NewsArticle;
use super::Summary;
use super::Tweet;

impl Summary for NewsArticle {
  fn summarization_author(&self) -> String {
    format!("{}", self.author)
  }
}

impl Summary for Tweet {
  fn summarization_author(&self) -> String {
    format!("@{}", self.username)
  }
}

// fn notify(item: &impl Summary) {
//   println!("Breaking news! {}", item.summarize());
// }

fn notify<T: Summary>(item: &T) {
  println!("Breaking news! {}", item.summarize());
}

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}
fn _some_function<T, U>(_t: &T, _u: &U)
where
  T: Display + Clone,
  U: Clone + Debug,
{
  // Some codes here
}

pub fn run() {
  let tweet = Tweet {
    username: "@huajianmao".to_string(),
    content: "Hello world!".to_string(),
    reply: false,
    retweet: false,
  };

  let article = NewsArticle {
    author: "Huajian Mao".to_string(),
    headline: "The sky is Falling".to_string(),
    content: "This sky is not actually falling.".to_string(),
  };

  notify(&tweet);
  notify(&article);
}
