fn get_largest<T: PartialOrd + Copy>(numbers: Vec<T>) -> T {
  let mut largest = numbers[0];
  for number in numbers {
    if number > largest {
      largest = number;
    }
  }

  largest
}

pub fn run() {
  let numbers = vec![34, 50, 25, 100, 65];
  let largest = get_largest(numbers);
  println!("The largest number is {}", largest);

  let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];
  let largest = get_largest(numbers);
  println!("The largest number is {}", largest);

  let chars = vec!['A', 'T', 'C', 'G', 'a', 't', 'c', 'g'];
  let largest = get_largest(chars);
  println!("The largest number is {}", largest);

  let numbers = vec![102.0, 34., 6000.0, 89., 54., 2., 43., 8.];
  let largest = get_largest(numbers);
  println!("The largest number is {}", largest);
}
