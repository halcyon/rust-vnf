#![allow(unused)]

use crate::column::Type;
use std::u8;

pub const SIGNATURE: [u8; 11] = [78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0];
pub const VERSION: [u8; 2] = [1, 0];
pub const FILLER: [u8; 1] = [0];

pub fn to_header(types: &[Type]) -> Vec<u8> {
    let header_area_length = ((4 * types.len() + 5) as u32).to_le_bytes();

    let number_of_columns = (types.len() as u16).to_le_bytes();

    let mut vec: Vec<u8> = Vec::with_capacity(
        SIGNATURE.len()
            + header_area_length.len()
            + VERSION.len()
            + FILLER.len()
            + number_of_columns.len()
            + std::mem::size_of::<u32>() * types.len(),
    );

    vec.extend_from_slice(&SIGNATURE);
    vec.extend_from_slice(&header_area_length);
    vec.extend_from_slice(&VERSION);
    vec.extend_from_slice(&FILLER);
    vec.extend_from_slice(&number_of_columns);

    for t in types.iter() {
        vec.extend_from_slice(&u32::from(t).to_le_bytes())
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_no_columns() {
        assert_eq!(
            vec![
                78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, // SIGNATURE
                5, 0, 0, 0, // header_area_length
                1, 0, // VERSION
                0, // FILLER
                0, 0 // number_of_columns
            ],
            to_header(&[])
        );
    }

    #[test]
    fn new_file_header_with_column() {
        assert_eq!(
            vec![
                78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, // SIGNATURE
                9, 0, 0, 0, // header_area_length
                1, 0, // VERSION
                0, // FILLER
                1, 0, // number_of_columns
                255, 255, 255, 255, // column_widths
            ],
            to_header(&[Type::VarChar])
        );
    }

    #[test]
    fn new_file_header_with_two_columns() {
        assert_eq!(
            vec![
                78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, // SIGNATURE
                13, 0, 0, 0, // header_area_length
                1, 0, // VERSION
                0, // FILLER
                2, 0, // number_of_columns
                255, 255, 255, 255, // column_widths
                4, 0, 0, 0, // column_widths
            ],
            to_header(&[Type::VarChar, Type::Char { len: 4 }])
        );
    }

    #[test]
    fn new_file_header_with_255_columns() {
        let mut expected: Vec<u8> = Vec::new();
        let header_area_length: [u8; 4] = [1, 4, 0, 0];
        let number_of_columns: [u8; 2] = [255, 0];
        let column_widths: Vec<u8> = vec![u8::MAX; 1020];
        expected.extend(&SIGNATURE);
        expected.extend(&header_area_length);
        expected.extend(&VERSION);
        expected.extend(&FILLER);
        expected.extend(&number_of_columns);
        expected.extend(column_widths);
        assert_eq!(expected, to_header(&[Type::VarBinary; 255]));
    }

    #[test]
    fn new_file_header_with_256_columns() {
        let mut expected: Vec<u8> = Vec::new();
        let header_area_length: [u8; 4] = [5, 4, 0, 0];
        let number_of_columns: [u8; 2] = [0, 1];
        let column_widths: Vec<u8> = vec![u8::MAX; 1024];
        expected.extend(&SIGNATURE);
        expected.extend(&header_area_length);
        expected.extend(&VERSION);
        expected.extend(&FILLER);
        expected.extend(&number_of_columns);
        expected.extend(column_widths);
        assert_eq!(expected, to_header(&[Type::VarBinary; 256]));
    }

    #[test]
    fn new_file_header_with_257_columns() {
        let mut expected: Vec<u8> = Vec::new();
        let header_area_length: [u8; 4] = [9, 4, 0, 0];
        let number_of_columns: [u8; 2] = [1, 1];
        let column_widths: Vec<u8> = vec![u8::MAX; 1028];
        expected.extend(&SIGNATURE);
        expected.extend(&header_area_length);
        expected.extend(&VERSION);
        expected.extend(&FILLER);
        expected.extend(&number_of_columns);
        expected.extend(column_widths);
        assert_eq!(expected, to_header(&[Type::VarBinary; 257]));
    }
}
