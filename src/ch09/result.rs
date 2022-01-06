use std::fs::File;
use std::io::{self, /*ErrorKind,*/ Read};

pub fn run() {
  let file_name = "/tmp/hello.txt";
  // let file = File::open(file_name);
  // let _file = match file {
  //   Ok(f) => f,
  //   Err(error) => match error.kind() {
  //     ErrorKind::NotFound => match File::create(file_name) {
  //       Ok(fc) => fc,
  //       Err(e) => panic!("Problem creating the file: {:?}", e),
  //     },
  //     other_error => panic!("Problem opening the file: {:?}", other_error),
  //   },
  // };

  // // Reulst.unwrap_or_else();
  // // ========================
  // let _file = File::open(file_name).unwrap_or_else(|error| {
  //   match error.kind() {
  //     ErrorKind::NotFound => File::create(file_name).unwrap_or_else(|error| {
  //       panic!("Problem creating the file: {:?}", error);
  //     }),
  //     _ => panic!("Problem opening the file: {:?}", error),
  //   }
  // });

  // let create_or_panic = |file_name| {
  //   File::create(file_name).unwrap_or_else(|error| {
  //     panic!("Problem creating the file: {:?}", error);
  //   })
  // };
  // let _file = File::open(file_name).unwrap_or_else(|error| {
  //   match error.kind() {
  //     ErrorKind::NotFound => create_or_panic(file_name),
  //     _ => panic!("Problem opening the file: {:?}", error),
  //   }
  // });

  // // Result.expect();
  let _file = File::open(file_name).expect("Failed to open the file!");

  // Error propagation, returning the error to the caller
}

// ? operator can only be used in the function which returns a Result type.
// So it can not be used in the main or make the main return a Result type.
fn _read_username_from_file() -> Result<String, io::Error> {
  let mut username = String::new();

  // let file = File::open("hello.txt");
  // let mut file = match file {
  //   Ok(f) => f,
  //   Err(e) => return Err(e),
  // };

  // <----- Attention! to the ?
  // ? will return the error to the caller if failed
  let mut file = File::open("hello.txt")?;

  // match file.read_to_string(&mut username) {
  //   Ok(_) => Ok(username),
  //   Err(e) => Err(e),
  // }
  file.read_to_string(&mut username)?;
  Ok(username)

  // // Simplified version
  // File::open("hello.txt")?.read_to_string(&mut username)?;
  // Ok(username)

  // // Final version
  // fs::read_to_string("hello.txt")   <---- This is the same as the simplified one
}
