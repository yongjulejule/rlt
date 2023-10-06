use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::{Read, Write};

pub fn compress(content: &[u8]) -> Vec<u8> {
  let mut compressed = ZlibEncoder::new(Vec::new(), Compression::default());
  compressed
    .write_all(content)
    .expect("failed to write to compressed file");
  return compressed
    .finish()
    .expect("failed to finish compressed file");
}

pub fn decompress(content: &[u8]) -> Vec<u8> {
  let mut decompressed = Vec::new();
  let mut decoder = ZlibDecoder::new(content);
  decoder
    .read_to_end(&mut decompressed)
    .expect("failed to read compressed file");
  return decompressed;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_compress() {
    let content = "test-content".as_bytes();
    let compressed = compress(content);
    let decompressed = decompress(&compressed);
    assert_eq!(decompressed, content);
  }
}
