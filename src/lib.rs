pub mod column;
pub mod date;
mod file;
pub mod header;
pub mod row;
pub mod value;

use header::to_header;
use std::io;
use value::Value;

struct Writer<'a> {
    types: &'a [column::Type],
    file_header: Vec<u8>,
    row_header_fill: Vec<u8>,
    null_value_bit_field: Vec<u8>,
}

impl<'a> Writer<'a> {
    fn new(types: &[column::Type]) -> Writer {
        let null_value_bitfield_len = types.len() / 8 + if types.len() % 8 > 0 { 1 } else { 0 };
        let row_size_len: usize = 4;
        Writer {
            types,
            file_header: to_header(types),
            row_header_fill: vec![0u8; row_size_len + null_value_bitfield_len],
            null_value_bit_field: vec![0u8; null_value_bitfield_len],
        }
    }

    fn write_header<'b, W: io::Write>(&self, out: &'b mut W) {
        out.write(self.file_header.as_slice());
    }

    fn write_row<'b, W: io::Write + io::Seek>(&self, out: &'b mut W, values: &[Value]) {
        out.write(&self.row_header_fill);
        let row_len: usize = values.iter()
            .map(|value| value.write(out))
            .map(|result| match result {Ok(len) => len, _ => 0})
            .sum();
        let pos = (self.row_header_fill.len() + row_len) as i64;
        out.seek(io::SeekFrom::Current(-pos));
        out.write(&(row_len as u32).to_le_bytes());
        out.write(&self.null_value_bit_field);
        out.seek(io::SeekFrom::Current(pos));
    }
}

// include bytes

#[cfg(test)]
mod tests {
    use super::*;
    use column::Type;
    use value::Value;
    use std::io::Cursor;

    #[test]
    fn write_header() {
        let writer = Writer::new(&[Type::Integer]);
        let mut out: Cursor<Vec<u8>> = Cursor::new(vec![]);
        writer.write_header(&mut out);
        writer.write_row(&mut out, &[Value::Integer(4), Value::Null]);

        assert_eq!(
            &vec![
                78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, // SIGNATURE
                9, 0, 0, 0, // header_area_length
                1, 0, // VERSION
                0, // FILLER
                1, 0, // number_of_columns
                8, 0, 0, 0, // size of column 1
                8, 0, 0, 0, // size of row 1
                0, // null value bit field
                4, 0, 0, 0, 0, 0, 0, 0, // row 1, column 1
            ],
            out.get_ref()
        )
    }
}
