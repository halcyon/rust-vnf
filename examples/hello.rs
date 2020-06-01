extern crate vnf;

use std::io::Cursor;
use vnf::column::Type;
use vnf::column::Value;
use vnf::VnfWriter;

fn main() {
    let mut writer = VnfWriter::new(&[
        Type::Integer,
        Type::Boolean,
        Type::Char { len: 4 },
        Type::Boolean,
    ]);
    let mut out: Cursor<Vec<u8>> = Cursor::new(vec![]);
    writer.write_file_header(&mut out);
    writer.write_row(
        &mut out,
        &[
            Value::Integer(4),
            Value::Boolean(true),
            Value::Char("Fred"),
            Value::Null,
        ],
    );

    assert_eq!(
        &vec![
            78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, // SIGNATURE
            21, 0, 0, 0, // header_area_length
            1, 0, // VERSION
            0, // FILLER
            4, 0, // number_of_columns
            8, 0, 0, 0, // size of column 1
            1, 0, 0, 0, // size of column 2
            4, 0, 0, 0, // size of column 3
            1, 0, 0, 0, // size of column 4
            13, 0, 0, 0,          // size of row 1
            0b00010000, // null value bit field
            4, 0, 0, 0, 0, 0, 0, 0, // row 1, column 1
            1, // row 1, column 2
            0x46, 0x72, 0x65, 0x64, // row 1, column 3
        ],
        out.get_ref()
    );
    println!("Hello world!");
}
