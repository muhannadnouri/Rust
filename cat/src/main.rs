use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let file_name = &args[1];
  let file_content = fs::read_to_string(file_name).expect("Unable to read file");
  println!("{}", file_content);
}
