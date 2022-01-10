use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
  println!("calculating slowly...");
  thread::sleep(Duration::from_secs(1));
  intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
  if intensity < 25 {
    println!(
      "Today, do {} pushups!",
      simulated_expensive_calculation(intensity)
    );
    println!(
      "Next, do {} situps!",
      simulated_expensive_calculation(intensity)
    );
  } else {
    if random_number == 3 {
      println!("Take a break today! Remeber to stay hydrated!");
    } else {
      println!(
        "Today, run for {} minutes!",
        simulated_expensive_calculation(intensity)
      );
    }
  }
}

pub fn run() {
  println!("Closures::original ...");

  let simulated_insensity = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_insensity, simulated_random_number);
}
