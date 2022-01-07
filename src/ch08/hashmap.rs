use std::collections::HashMap;

pub fn run() {
  let blue = String::from("Blue");
  let yellow = String::from("Yellow");

  let mut scores = HashMap::new();
  scores.insert(blue, 10);
  scores.insert(yellow, 50);

  match scores.get("Blue") {
    Some(i) => println!("Score of Blue is {}", i),
    None => println!("Score of Blue not found!"),
  }

  for (k, v) in &scores {
    println!("{}: {}", k, v);
  }
  scores.iter().for_each(|(k, v)| println!("{}: {}", k, v));

  scores.insert(String::from("Blue"), -90);
  scores.entry(String::from("None")).or_insert(80);

  scores.iter().for_each(|(k, v)| println!("{}: {}", k, v));

  update_value_based_on_old_data();
}

fn update_value_based_on_old_data() {
  let text = "hello world wonderful world";

  let mut map = HashMap::new();
  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }
  println!("{:?}", map);
}
