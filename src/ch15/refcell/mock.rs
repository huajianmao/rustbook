pub trait Messenger {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
  T: Messenger,
{
  pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
    LimitTracker {
      messenger,
      value: 0,
      max,
    }
  }

  pub fn set_value(&mut self, value: usize) {
    self.value = value;

    let ratio = self.value as f64 / self.max as f64;
    if ratio >= 1.0 {
      self.messenger.send("Error: You are over your quota!");
    } else if ratio >= 0.9 {
      self
        .messenger
        .send("Urgent warning: You've used up over 90% of quota!");
    } else if ratio >= 0.75 {
      self
        .messenger
        .send("Warning: You've used up over 75% of your quota!");
    }
  }
}

pub fn run() {
  println!("RefCell::MockObject");

  it_sends_an_over_75_percent_warning_message();
}

use std::cell::RefCell;
struct MockMessenger {
  sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
  fn new() -> MockMessenger {
    MockMessenger {
      sent_messages: RefCell::new(vec![]),
    }
  }
}

impl Messenger for MockMessenger {
  fn send(&self, msg: &str) {
    self.sent_messages.borrow_mut().push(String::from(msg));

    // // Fail at runtime
    // let mut one_borrow = self.sent_messages.borrow_mut();
    // let mut two_borrow = self.sent_messages.borrow_mut();
    // one_borrow.push(String::from(msg));
    // two_borrow.push(String::from(msg));
  }
}

fn it_sends_an_over_75_percent_warning_message() {
  let mock_messenger = MockMessenger::new();
  let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
  limit_tracker.set_value(80);
  assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);

  println!("it_sends_an_over_75_percent_warning_message runs OK!");
}
