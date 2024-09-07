use std::fs::File;
use std::io::Read;

pub fn read_file_bytes(path: &str) -> Result<Vec<u8>, std::io::Error> {
  let mut file = File::open(path)?;
  let mut contents: Vec<u8> = vec![];
  file.read_to_end(&mut contents)?;
  Ok(contents)
}
