use super::NewsArticle;
use super::Tweet;

impl Summary for NewsArticle {
  fn summarization(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.content)
  }
}

impl Summary for Tweet {
  fn summarization(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

pub trait Summary {
  fn summarization(&self) -> String;
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
