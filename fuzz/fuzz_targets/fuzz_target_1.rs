#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate insta;

fuzz_target!(|data: &[u8]| {
  match insta::fuzz(data) {
    Ok(_) => panic!("sorted B)"),
    Err(err) => println!("{}", err),
  };
});
