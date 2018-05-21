extern crate insta;

use insta::fuzz;
use std::fs::File;

fn main() {
  let file = File::open("./pic.jpg").expect("failed to open file");
  match fuzz(file) {
    Ok(_) => panic!("sorted B)"),
    Err(err) => println!("{}", err),
  };
}