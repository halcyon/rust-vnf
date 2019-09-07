use std::convert::From;
use std::fmt;

// enum ColumnTypes {
//    INTEGER = 0x08,
//    FLOAT = 0x08,
//    CHAR { length: i32 },
//    VARCHAR = 0xFF_FF_FF_FF,
//    BOOLEAN = 0x01,
//    DATE = 0x08,
//    TIMESTAMP = 0x08,
//    TIMESTAMPTZ = 0x08,
//    TIME = 0x08,
//    TIMETZ = 0x08,
//    VARBINARY = 0xFF_FF_FF_FF,
//    BINARY = 0x03,
//    NUMERIC {precision: i32, scale: i32},
//    INTERVAL = 0x08
// }

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
            ColumnTypes::VarChar | ColumnTypes::VarBinary => std::u32::MAX,
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
    column_widths: Vec<i32>,
}

impl FileHeader {
    pub fn new(column_widths: Vec<i32>) -> FileHeader {
        FileHeader {
            signature: [78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0],
            header_area_length: ((4 * column_widths.len() + 5) as u32).to_le_bytes(),
            version: [1, 0],
            filler: 0,
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
        assert_eq!(vec!(78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, 05, 0, 0, 0, 1, 0, 0, 0, 0),
                   Vec::from(FileHeader::new(vec!())));
    }

    #[test]
    fn new_file_header_with_one_column() {
        assert_eq!(vec!(78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, 09, 0, 0, 0, 1, 0, 0, 1, 0,
                        255, 255, 255, 255),
                   Vec::from(FileHeader::new(vec!(-1))));
    }

    #[test]
    fn new_file_header_with_two_columns() {
        assert_eq!(vec!(78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, 13, 0, 0, 0, 1, 0, 0, 2, 0,
                        255, 255, 255, 255,
                        4, 0, 0, 0),
                   Vec::from(FileHeader::new(vec!(-1, 4))));
    }

    #[test]
    fn new_file_header_with_255_columns() {
        let mut ex255 = vec!(78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, 1, 4, 0, 0, 1, 0, 0, 255, 0);
        ex255.extend(vec!(255; 4 * 255));
        assert_eq!(ex255, Vec::from(FileHeader::new(vec!(-1; 255))));
    }

    #[test]
    fn new_file_header_with_256_columns() {
        let mut ex256 = vec!(78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, 5, 4, 0, 0, 1, 0, 0, 0, 1);
        ex256.extend(vec!(255; 4 * 256));
        assert_eq!(ex256, Vec::from(FileHeader::new(vec!(-1; 256))));
    }

    #[test]
    fn new_file_header_with_257_columns() {
        let mut ex257 = vec!(78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, 9, 4, 0, 0, 1, 0, 0, 1, 1);
        ex257.extend(vec!(255; 4 * 257));
        assert_eq!(ex257, Vec::from(FileHeader::new(vec!(-1; 257))));
    }

    #[test]
    fn enum_stuff() {
        println!("{:X}", ColumnTypes::Binary.as_bytes());
        println!("{:X}", ColumnTypes::VarBinary.as_bytes());
        println!("{:X}", ColumnTypes::Char(6).as_bytes());
    }


}
