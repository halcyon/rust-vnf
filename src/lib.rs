use std::convert::From;
use std::fmt;
use std::u8;
use std::u32;

pub const SIGNATURE: [u8; 11] = [78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0];
pub const VERSION: [u8; 2] = [1, 0];
pub const FILLER: u8 = 0;

enum ColumnTypes {
    Integer,
    Float,
    Char(u32),
    VarChar,
    Boolean,
    Date,
    Timestamp,
    TimestampTz,
    Time,
    TimeTz,
    VarBinary,
    Binary,
    // Numeric {precision: i32, scale: i32},
    Interval
}

impl ColumnTypes {
    fn as_bytes(&self) -> u32 {
        match *self {
            ColumnTypes::Integer |
            ColumnTypes::Float |
            ColumnTypes::Date |
            ColumnTypes::Timestamp |
            ColumnTypes::TimestampTz |
            ColumnTypes::Time |
            ColumnTypes::TimeTz |
            ColumnTypes::Interval => 8u32,

            ColumnTypes::Char(length)  => length,
            ColumnTypes::VarChar | ColumnTypes::VarBinary => u32::MAX,
            ColumnTypes::Boolean => 1u32,
            ColumnTypes::Binary => 3u32
        }

    }
}

struct FileHeader {
    signature: [u8; 11],
    header_area_length: [u8; 4],
    version: [u8; 2],
    filler: u8,
    number_of_columns: [u8; 2],
    column_widths: Vec<u32>,
}

impl FileHeader {
    pub fn new(column_widths: Vec<u32>) -> FileHeader {
        FileHeader {
            signature: SIGNATURE,
            header_area_length: ((4 * column_widths.len() + 5) as u32).to_le_bytes(),
            version: VERSION,
            filler: FILLER,
            number_of_columns: (column_widths.len() as u16).to_le_bytes(),
            column_widths: column_widths,
        }
    }
}

impl fmt::Display for FileHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "signature: {:X?}", self.signature)?;
        writeln!(f, "header_area_length: {:X?}", self.header_area_length)?;
        writeln!(f, "version: {:X?}", self.version)?;
        writeln!(f, "filler: {:X?}", self.filler)?;
        writeln!(f, "number_of_columns: {:X?}", self.number_of_columns)
    }
}

impl From<FileHeader> for Vec<u8> {
    fn from(header: FileHeader) -> Self {
        let mut vec: Vec<u8> = Vec::new();
        vec.extend(header.signature.iter());
        vec.extend(header.header_area_length.iter());
        vec.extend(header.version.iter());
        vec.push(header.filler);
        vec.extend(header.number_of_columns.iter());
        for w in header.column_widths {
            vec.extend(&w.to_le_bytes())
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_file_header_with_no_columns() {
        let mut expected: Vec<u8> = Vec::new();
        let header_area_length: [u8; 4
] = [5, 0, 0, 0];
        let number_of_columns: [u8; 2] = [0, 0];
        expected.extend(&SIGNATURE);
        expected.extend(&header_area_length);
        expected.extend(&VERSION);
        expected.push(FILLER);
        expected.extend(&number_of_columns);
        assert_eq!(expected,
                   Vec::from(FileHeader::new(vec!())));
    }

    #[test]
    fn new_file_header_with_one_column() {
        let mut expected: Vec<u8> = Vec::new();
        let header_area_length: [u8; 4] = [9, 0, 0, 0];
        let number_of_columns: [u8; 2] = [1, 0];
        let column_widths: Vec<u8> = vec!(u8::MAX; 4);
        expected.extend(&SIGNATURE);
        expected.extend(&header_area_length);
        expected.extend(&VERSION);
        expected.push(FILLER);
        expected.extend(&number_of_columns);
        expected.extend(column_widths);
        assert_eq!(expected,
                   Vec::from(FileHeader::new(vec!(u32::MAX))));
    }

    #[test]
    fn new_file_header_with_two_columns() {
        let mut expected: Vec<u8> = Vec::new();
        let header_area_length: [u8; 4] = [13, 0, 0, 0];
        let number_of_columns: [u8; 2] = [2, 0];
        let column_widths: Vec<u8> = vec!(u8::MAX, u8::MAX, u8::MAX, u8::MAX,
                                          4, 0, 0, 0);
        expected.extend(&SIGNATURE);
        expected.extend(&header_area_length);
        expected.extend(&VERSION);
        expected.push(FILLER);
        expected.extend(&number_of_columns);
        expected.extend(column_widths);
        assert_eq!(expected,
                   Vec::from(FileHeader::new(vec!(u32::MAX, 4))));
    }

    #[test]
    fn new_file_header_with_255_columns() {
        let mut expected: Vec<u8> = Vec::new();
        let header_area_length: [u8; 4] = [1, 4, 0, 0];
        let number_of_columns: [u8; 2] = [255, 0];
        let column_widths: Vec<u8> = vec!(u8::MAX; 1020);
        expected.extend(&SIGNATURE);
        expected.extend(&header_area_length);
        expected.extend(&VERSION);
        expected.push(FILLER);
        expected.extend(&number_of_columns);
        expected.extend(column_widths);
        assert_eq!(expected,
                   Vec::from(FileHeader::new(vec!(u32::MAX; 255))));
    }

    #[test]
    fn new_file_header_with_256_columns() {
        let mut expected: Vec<u8> = Vec::new();
        let header_area_length: [u8; 4] = [5, 4, 0, 0];
        let number_of_columns: [u8; 2] = [0, 1];
        let column_widths: Vec<u8> = vec!(u8::MAX; 1024);
        expected.extend(&SIGNATURE);
        expected.extend(&header_area_length);
        expected.extend(&VERSION);
        expected.push(FILLER);
        expected.extend(&number_of_columns);
        expected.extend(column_widths);
        assert_eq!(expected,
                   Vec::from(FileHeader::new(vec!(u32::MAX; 256))));
    }

    #[test]
    fn new_file_header_with_257_columns() {
        let mut expected: Vec<u8> = Vec::new();
        let header_area_length: [u8; 4] = [9, 4, 0, 0];
        let number_of_columns: [u8; 2] = [1, 1];
        let column_widths: Vec<u8> = vec!(u8::MAX; 1028);
        expected.extend(&SIGNATURE);
        expected.extend(&header_area_length);
        expected.extend(&VERSION);
        expected.push(FILLER);
        expected.extend(&number_of_columns);
        expected.extend(column_widths);
        assert_eq!(expected,
                   Vec::from(FileHeader::new(vec!(u32::MAX; 257))));
    }

    // #[test]
    // fn new_file_header() {
    //     assert_eq!(vec!(78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, 05, 0, 0, 0, 1, 0, 0, 0, 0),
    //                Vec::from(FileHeader::new(vec!())));
    // }

    // fn enum_stuff() {
    //     println!("{:X}", ColumnTypes::Binary.as_bytes());
    //     println!("{:X}", ColumnTypes::VarBinary.as_bytes());
    //     println!("{:X}", ColumnTypes::Char(6).as_bytes());
    // }
}
