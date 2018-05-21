extern crate jpeg_decoder as jpeg;

use jpeg::PixelFormat;

use std::io::{BufReader, Read};

pub enum Error {
  Jpeg(jpeg::Error),
  FormatMismatch,
  MetricMismatch,
}

pub fn fuzz(data: impl Read) -> Result<f64, Error> {
  let required_width = 32;
  let required_height = 32;
  let mut decoder = jpeg::Decoder::new(BufReader::new(data));
  match decoder.read_info() {
    Ok(_) => {}
    Err(err) => return Err(Error::Jpeg(err)),
  };
  let info = match decoder.info() {
    Some(info) => info,
    None => return Err(Error::FormatMismatch),
  };
  if info.width != required_width || info.height != required_height
    || info.pixel_format != PixelFormat::RGB24
  {
    return Err(Error::FormatMismatch);
  };
  let pixels = match decoder.decode() {
    Ok(pixels) => pixels,
    Err(err) => return Err(Error::Jpeg(err)),
  };
  let mut reds: u64 = 0;
  for pixel in pixels.chunks(3) {
    if let &[r, g, b] = pixel {
      if r > 50 && r as u16 >= (2 * g as u16) && r as u16 >= (2 * b as u16) {
        reds += 1;
      }
    }
  }
  let size = required_width * required_height;
  let metric = reds as f64 / size as f64;
  if metric > 0.5 {
    Ok(metric)
  } else {
    Err(Error::MetricMismatch)
  }
}
