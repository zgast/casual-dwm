use std::fs::File;
use std::io::{Read, Write};

pub fn write_to_file(path: &str, contents : String) {

    let mut f = std::fs::OpenOptions::new().write(true).open(path).unwrap();
    f.write_all(contents.as_bytes());
    f.flush();
}