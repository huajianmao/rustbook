mod draw;

use draw::{Button, Screen, SelectBox};

pub fn run() {
  println!("Chapter 17::Trait Objects");

  let button = Button {
    width: 100,
    height: 24,
    label: String::from("Start"),
  };

  let select_box = SelectBox {
    width: 100,
    height: 100,
    options: vec![
      String::from("Yes"),
      String::from("No"),
      String::from("Maybe"),
    ],
  };

  let screen = Screen {
    components: vec![Box::new(select_box), Box::new(button)],
  };

  screen.run();
}
