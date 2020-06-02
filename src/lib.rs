#![allow(unused)]

pub mod column;
pub mod date;
mod file;
pub mod header;
pub mod row;

use column::Value;
use header::to_header;
use std::{cmp::min, io};

const BIT_POSITION: [u8; 8] = [
    0b1000_0000,
    0b0100_0000,
    0b0010_0000,
    0b0001_0000,
    0b0000_1000,
    0b0000_0100,
    0b0000_0010,
    0b0000_0001,
];

/// Convert a slice of column values to a null value bit field.
/// Each byte represents 8 columns, a high bit means the column
/// value is NULL.
fn build_null_value_bit_field(values: &[column::Value]) -> Vec<u8> {
    let mut buffer = Vec::<u8>::new();
    push_null_values(&mut buffer, values);
    buffer
}

fn push_null_values(buffer: &mut Vec<u8>, values: &[column::Value]) {
    values
        .iter()
        .enumerate()
        .map(|(i, value)| {
            (
                i,
                match value {
                    column::Value::Null => BIT_POSITION[i % 8],
                    _ => 0,
                },
            )
        })
        .for_each(|(i, bit)| {
            if i % 8 == 0 {
                buffer.push(bit)
            } else {
                let j = buffer.len() - 1;
                buffer[j] = buffer[j] | bit;
            }
        })
}

fn push_row_data(buffer: &mut Vec<u8>, types: &[column::Type], values: &[column::Value]) {
    values
        .iter()
        .enumerate()
        .map(|(i, v)| (types[i], v))
        .for_each(|(t, v)| t.append(buffer, v))
}

fn build_row_data(types: &[column::Type], values: &[column::Value]) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    push_row_data(&mut buf, types, values);
    buf
}

pub struct VnfWriter<'a> {
    buf: Vec<u8>,
    column_types: &'a [column::Type],
    file_header: Vec<u8>,
}

impl<'a> VnfWriter<'a> {
    pub fn new(column_types: &[column::Type]) -> VnfWriter {
        VnfWriter {
            buf: Vec::<u8>::new(),
            column_types,
            file_header: to_header(column_types),
        }
    }

    pub fn write_file_header<W: io::Write>(&self, out: &mut W) -> io::Result<usize> {
        out.write(self.file_header.as_slice())
    }

    pub fn write_row<'b, W: io::Write>(
        &mut self,
        out: &'b mut W,
        values: &[column::Value],
    ) -> io::Result<usize> {
        self.buf.clear();
        self.buf.extend_from_slice(&[0, 0, 0, 0]);
        push_null_values(&mut self.buf, values);
        let row_data_start = self.buf.len();
        push_row_data(&mut self.buf, self.column_types, &values);
        let row_data_len = self.buf.len() - row_data_start;
        let le_bytes = (row_data_len as u32).to_le_bytes();
        le_bytes
            .iter()
            .enumerate()
            .for_each(|(i, b)| self.buf[i] = *b);
        out.write(&self.buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use column::Type;
    use column::Value;
    use std::io::Cursor;

    #[test]
    fn null_value_bit_field() {
        assert_eq!(
            vec![0b0000_0000],
            build_null_value_bit_field(&[column::Value::Boolean(true)])
        );
        assert_eq!(
            vec![0b1000_0000],
            build_null_value_bit_field(&[column::Value::Null])
        );
        assert_eq!(
            vec![0b1100_0000],
            build_null_value_bit_field(&[column::Value::Null, column::Value::Null])
        );
        assert_eq!(
            vec![0b1111_1111],
            build_null_value_bit_field(&[column::Value::Null; 8])
        );
        assert_eq!(
            vec![0b1111_1111, 0b1000_0000],
            build_null_value_bit_field(&[column::Value::Null; 9])
        );

        assert_eq!(
            vec![0b1101_1000, 0b1000_0000],
            build_null_value_bit_field(&[
                column::Value::Null,
                column::Value::Null,
                column::Value::Boolean(true),
                column::Value::Null,
                column::Value::Null,
                column::Value::Integer(8),
                column::Value::Char("ted"),
                column::Value::Char("bill"),
                column::Value::Null,
            ])
        )
    }

    #[rustfmt::skip]
    #[test]
    fn write_vnf() {
        let mut writer = VnfWriter::new(&[
            Type::Integer, Type::Boolean, Type::Char { len: 4 }, Type::Boolean,
            Type::Boolean, Type::Boolean, Type::Boolean, Type::Boolean,
            Type::Boolean,
        ]);
        let mut out: Cursor<Vec<u8>> = Cursor::new(vec![]);
        writer.write_file_header(&mut out);
        writer.write_row(
            &mut out,
            &[
                Value::Integer(4), Value::Boolean(true), Value::Char("Fred"), Value::Null,
                Value::Null, Value::Null, Value::Null, Value::Null,
                Value::Null,
            ],
        );

        assert_eq!(
            &vec![78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, // SIGNATURE
                  41, 0, 0, 0, // header_area_length (number_of_columns * 4) + 5
                  1, 0, // VERSION
                  0, // FILLER
                  9, 0, // number_of_columns
                  8, 0, 0, 0, // size of column 1
                  1, 0, 0, 0, // size of column 2
                  4, 0, 0, 0, // size of column 3
                  1, 0, 0, 0, // size of column 4
                  1, 0, 0, 0, // size of column 5
                  1, 0, 0, 0, // size of column 6
                  1, 0, 0, 0, // size of column 7
                  1, 0, 0, 0, // size of column 8
                  1, 0, 0, 0, // size of column 9
                  13, 0, 0, 0, // size of row 1
                  0b0001_1111, // null value bit field
                  0b1000_0000, // null value bit field
                  4, 0, 0, 0, 0, 0, 0, 0, // row 1, column 1
                  1, // row 1, column 2
                  0x46, 0x72, 0x65, 0x64, // row 1, column 3
            ],
            out.get_ref()
        )
    }

    #[test]
    fn row_data() {
        assert_eq!(
            Vec::<u8>::new(),
            build_row_data(&[column::Type::Boolean], &[column::Value::Null])
        );
        assert_eq!(
            vec![1u8],
            build_row_data(&[column::Type::Boolean], &[column::Value::Boolean(true)])
        );
        assert_eq!(
            vec![1u8, 65, 66, 67, 68],
            build_row_data(
                &[column::Type::Boolean, column::Type::Char { len: 4 }],
                &[column::Value::Boolean(true), column::Value::Char("ABCDE")],
            )
        );
        assert_eq!(
            vec![1u8, 65, 66, 67, 68],
            build_row_data(
                &[column::Type::Boolean, column::Type::Char { len: 4 }],
                &[column::Value::Boolean(true), column::Value::Char("ABCDE")],
            )
        );
        assert_eq!(
            vec![1u8, 65, 66, 67, 68],
            build_row_data(
                &[
                    column::Type::Boolean,
                    column::Type::Integer,
                    column::Type::Char { len: 4 }
                ],
                &[
                    column::Value::Boolean(true),
                    column::Value::Null,
                    column::Value::Char("ABCDE")
                ],
            )
        );
    }
}
