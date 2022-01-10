use std::thread;
use std::time::Duration;

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//   println!("calculating slowly...");
//   thread::sleep(Duration::from_secs(2));
//   intensity
// }

fn generate_workout(intensity: u32, random_number: u32) {
  // let expensive_result = simulated_expensive_calculation(intensity);
  let expensive_closure = |num| {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(1));
    num
  };

  if intensity < 25 {
    // println!("Today, do {} pushups!", expensive_result);
    // println!("Next, do {} situps!", expensive_result);
    println!("Today, do {} pushups!", expensive_closure(intensity));
    println!("Next, do {} situps!", expensive_closure(intensity));
  } else {
    if random_number == 3 {
      println!("Take a break today! Remeber to stay hydrated!");
    } else {
      // println!("Today, run for {} minutes!", expensive_result);
      println!("Today, run for {} minutes!", expensive_closure(intensity));
    }
  }

  let closure_could_only_have_one_concrete_type_inferred = |x| x;
  let _s = closure_could_only_have_one_concrete_type_inferred(String::from("Hello"));
  // let _fail = closure_could_only_have_one_concrete_type_inferred(5);
}

pub fn run() {
  println!("Closures::optimization ...");

  let simulated_insensity = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_insensity, simulated_random_number);
}
