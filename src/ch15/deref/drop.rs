struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data `{}`!", self.data);
  }
}

pub fn run() {
  println!("Deref::drop");

  let _c = CustomSmartPointer {
    data: String::from("my stuff"),
  };
  // _c.drop(); // not allowed to explicitly call drop()
  // drop(_c); // <--- Attention: This is allowed

  let _d = CustomSmartPointer {
    data: String::from("othr stuff"),
  };

  println!("CustomSmartPointer created.");
}
