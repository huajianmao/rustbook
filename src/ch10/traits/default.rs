use super::NewsArticle;
use super::Tweet;

impl Summary for NewsArticle {
  fn summarize_author(&self) -> String {
    format!("{}", self.author)
  }
  // fn summarization(&self) -> String {
  //   format!("{}, by {} ({})", self.headline, self.author, self.content)
  // }
}

impl Summary for Tweet {
  fn summarize_author(&self) -> String {
    format!("{}", self.username)
  }

  fn summarization(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

pub trait Summary {
  fn summarize_author(&self) -> String;

  fn summarization(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }
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

  println!("Tweet summary: {}", tweet.summarization());
  println!("Article summary: {}", article.summarization());
}
