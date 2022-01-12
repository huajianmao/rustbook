use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &T {
    return &self.0;
  }
}

fn hello(name: &str) {
  println!("Hello, {}!", name);
}

fn hello_vec(vec: &Vec<i32>) {
  println!("Hello, {:?}!", vec);
}

pub fn run() {
  println!("DeRef::DeRef");

  let x = 5;
  let y = Box::new(x);
  assert_eq!(x, *y);

  let z = MyBox::new(x);
  assert_eq!(x, *z);
  assert_eq!(x, *(z.deref()));

  let msg = MyBox::new(String::from("Rust"));
  hello(&msg); // implicit deref coercion // &MyBox<String> -- deref --> &String -- deref --> &str
  hello(&(msg.deref().deref())); // implicit deref coercion
  hello(&(*msg)[..]);

  let vec_box = MyBox::new(vec![1, 2, 3]);
  hello_vec(&vec_box); // &MyBox<Vec<i32>> -- deref --> &Vec<i32>

  println!("DeRef::DeRef runs OK!");
}
