use std::{
    fs,
    io::{self, Read},
};

pub fn read_file(path: &str) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}
