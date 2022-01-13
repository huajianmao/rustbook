pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
  pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

// // Only one type of Draw can be used but not mixure ones.
// pub struct Screen<T: Draw> {
//   pub components: Vec<T>,
// }

// impl<T: Draw> Screen<T> {
//   pub fn run(&self) {
//     for component in self.components.iter() {
//       component.draw();
//     }
//   }
// }

#[derive(Debug)]
pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self) {
    println!("Draw Button! {:?}", &self);
  }
}

#[derive(Debug)]
pub struct SelectBox {
  pub width: u32,
  pub height: u32,
  pub options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
    println!("Draw SelectBox! {:?}", &self);
  }
}
