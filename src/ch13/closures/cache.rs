use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cache<T, K, V>
where
  T: Fn(K) -> V,
  K: Eq + Hash + Copy,
  V: Copy,
{
  calculation: T,
  values: HashMap<K, V>,
}

impl<T, K, V> Cache<T, K, V>
where
  T: Fn(K) -> V,
  K: Eq + Hash + Copy,
  V: Copy,
{
  fn new(calculation: T) -> Cache<T, K, V> {
    Cache {
      calculation,
      values: HashMap::new(),
    }
  }

  fn value(&mut self, arg: K) -> V {
    match self.values.get(&arg) {
      Some(&v) => v,
      None => {
        let v = (self.calculation)(arg);
        self.values.insert(arg, v);
        v
      }
    }
  }
}

fn generate_workout(intensity: u32, random_number: u32) {
  let mut cache = Cache::new(|num| {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(1));
    num * 2
  });

  if intensity < 25 {
    println!("Today, do {} pushups!", cache.value(intensity));
    println!("Next, do {} situps!", cache.value(intensity));
  } else {
    if random_number == 3 {
      println!("Take a break today! Remeber to stay hydrated!");
    } else {
      println!("Today, run for {} minutes!", cache.value(intensity));
    }
  }
}

pub fn run() {
  println!("Closures::cache ...");

  let simulated_insensity = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_insensity, simulated_random_number);
}
