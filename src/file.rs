#![allow(unused)]

use std::fs::File;
use std::io::Write;

pub fn write_bytes(file_name: &str, bytes: &[u8]) -> Result<(), std::io::Error> {
    let mut file = File::create(file_name)?;
    file.write_all(bytes)
}
