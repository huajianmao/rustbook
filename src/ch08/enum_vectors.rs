pub fn run() {
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text("hello world".to_string()),
    SpreadsheetCell::Float(2.0),
  ];

  match &row[1] {
    SpreadsheetCell::Int(i) => println!("{}", i),
    SpreadsheetCell::Float(f) => println!("{}", f),
    SpreadsheetCell::Text(s) => println!("{}", s),
  }
}
