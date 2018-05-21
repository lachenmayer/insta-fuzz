extern crate insta;
extern crate jpeg_decoder as jpeg;

use std::io::BufReader;

use std::fs::{self, File};

// Generates an ImageMagick command to turn all valid JPEG files into a GIF.
fn main() {
  let dir = fs::read_dir("./fuzz/corpus/fuzz_target_1/").expect("bzzzoing");
  print!("convert ");
  for entry in dir {
    if let Ok(path) = entry {
      let file = File::open(path.path()).expect("failed to open file");
      let mut decoder = jpeg::Decoder::new(BufReader::new(file));
      match decoder.read_info() {
        Ok(_) => print!("{} ", path.path().to_str().unwrap()),
        Err(_err) => continue,
      };
    }
  }
  println!("corpus.gif");
  println!("ffmpeg -i corpus.gif -s 1024x1024 -sws_flags neighbor corpus.mp4");
}
