#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate insta;

use std::io::{self, Write};

use insta::Error;

fuzz_target!(|data: &[u8]| {
  match insta::fuzz(data) {
    Ok(_) => panic!("bzzt"),
    Err(Error::Jpeg(_err)) => print!("J"),
    Err(Error::FormatMismatch) => print!("F"),
    Err(Error::MetricMismatch) => print!("M"),
  };
  let _ = io::stdout().flush();
});
