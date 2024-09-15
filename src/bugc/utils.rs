use std::fs;
use std::io::{self, Read};
use std::path::Path;

pub fn read_file(path: &str) -> io::Result<String> {
  let mut file = fs::File::open(path)?;
  let mut buf = String::new();
  file.read_to_string(&mut buf)?;
  Ok(buf)
}

pub fn get_file_stem(string: &str) -> &str {
  Path::new(string).file_stem().unwrap().to_str().unwrap()
}
