#![allow(unused)]

use std::convert::From;
use std::u32;

pub struct Row {
    data_length: u32,
    null_bit_field: Vec<u8>,
    data: Vec<u8>,
}

impl Row {
    pub fn new(null_bit_field: Vec<u8>, data: Vec<u8>) -> Row {
        Row {
            data_length: data.len() as u32,
            null_bit_field,
            data,
        }
    }
}

impl From<Row> for Vec<u8> {
    fn from(row: Row) -> Self {
        let mut vec: Vec<u8> = Vec::new();
        vec.extend_from_slice(&row.data_length.to_le_bytes());
        vec.extend_from_slice(&row.null_bit_field);
        vec.extend_from_slice(&row.data);
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_row() {
        let mut expected: Vec<u8> = Vec::new();
        expected.extend_from_slice(&[10, 0, 0, 0]); // row length, excluding header
        expected.extend_from_slice(&[0b10000000]); // null bitfield
        expected.push(255); // column 1 value, true
        expected.extend_from_slice(&[5, 0, 0, 0]); // length of "hello"
        expected.extend_from_slice("hello".as_bytes()); // column 2 value

        let row = Row::new(
            vec![128],
            vec![
                255, 5, 0, 0, 0, 'h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8,
            ],
        );

        assert_eq!(expected, Vec::from(row));
    }
}
