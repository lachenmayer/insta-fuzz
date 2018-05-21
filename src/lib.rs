extern crate jpeg_decoder as jpeg;

use std::io::{BufReader, Read};

pub enum Error {
  Jpeg(jpeg::Error),
  FormatMismatch,
  MetricMismatch,
}

pub fn fuzz(data: impl Read) -> Result<f64, Error> {
  let mut decoder = jpeg::Decoder::new(BufReader::new(data));
  match decoder.read_info() {
    Ok(_) => {}
    Err(err) => return Err(Error::Jpeg(err)),
  };
  let info = match decoder.info() {
    Some(info) => info,
    None => return Err(Error::FormatMismatch),
  };
  if info.width != 64 && info.height != 64 {
    return Err(Error::FormatMismatch);
  };
  let pixels = match decoder.decode() {
    Ok(pixels) => pixels,
    Err(err) => return Err(Error::Jpeg(err)),
  };
  let mut total: u64 = 0;
  let size = pixels.len();
  for pixel in pixels {
    total += pixel as u64;
  }
  let average: f64 = total as f64 / size as f64;
  if average < 100.0 && average > 90.0 {
    Ok(average)
  } else {
    Err(Error::MetricMismatch)
  }
}
