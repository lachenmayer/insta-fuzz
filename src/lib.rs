extern crate jpeg_decoder as jpeg;

use std::error::Error;
use std::io::{BufReader, Read};
pub fn fuzz(data: impl Read) -> Result<(), impl Error> {
  let mut decoder = jpeg::Decoder::new(BufReader::new(data));
  decoder.read_info()?;
  let info = decoder
    .info()
    .ok_or_else(|| jpeg::Error::Format("no info".to_string()))?;
  if info.width != 32 && info.height != 32 {
    return Err(jpeg::Error::Format("not right size".to_string()));
  };
  let pixels = decoder.decode()?;
  let mut total: u64 = 0;
  let size = pixels.len();
  for pixel in pixels {
    total += pixel as u64;
  }
  let average: f64 = total as f64 / size as f64;
  if average < 100.0 && average > 90.0 {
    Ok(())
  } else {
    Err(jpeg::Error::Format("not the right image".to_string()))
  }
}
