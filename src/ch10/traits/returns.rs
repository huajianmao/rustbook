use super::NewsArticle;
use super::Summary;

fn returns_summarizable() -> impl Summary {
  NewsArticle {
    author: "Huajian Mao".to_string(),
    headline: "The sky is Falling".to_string(),
    content: "This sky is not actually falling.".to_string(),
  }
}

pub fn run() {
  println!("[Returns] {}", returns_summarizable().summarize());
}
