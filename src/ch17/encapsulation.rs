pub struct AverageCollection {
  list: Vec<i32>,
  average: f64,
}

impl AverageCollection {
  pub fn add(&mut self, num: i32) {
    self.list.push(num);
    self.update_average();
  }

  pub fn remove(&mut self) -> Option<i32> {
    let result = self.list.pop();
    if let Some(_) = result {
      self.update_average();
    }
    result
  }

  pub fn average(&self) -> f64 {
    self.average
  }

  fn update_average(&mut self) {
    let total: i32 = self.list.iter().sum();
    println!("total = {}, len = {}", total, self.list.len());
    self.average = total as f64 / self.list.len() as f64;
  }
}

pub fn run() {
  println!("Chapter 17::Encapsulation");

  let mut c = AverageCollection {
    list: vec![],
    average: 0.0,
  };

  let _ = &c.add(1);
  println!("Average is: {}", c.average());
  let _ = &c.remove();
  println!("Average is: {}", c.average());
}
