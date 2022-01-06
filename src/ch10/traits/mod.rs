mod blanket;
mod conditionally;
mod default;
mod params;
mod returns;
mod summary;

pub struct NewsArticle {
  pub author: String,
  pub headline: String,
  pub content: String,
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

pub trait Summary {
  fn summarization_author(&self) -> String;

  fn summarize(&self) -> String {
    format!("(Read more from {}...)", &self.summarization_author())
  }
}

pub fn run() {
  summary::run();
  default::run();
  params::run();
  returns::run();
  conditionally::run();
  blanket::run();
}
