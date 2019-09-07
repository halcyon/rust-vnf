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
}

impl FileHeader {
    pub fn new(number_of_columns: u16) -> FileHeader {
        FileHeader {
            signature: [0x4E, 0x41, 0x54, 0x49, 0x56, 0x45, 0x0A, 0xFF, 0x0D, 0x0A, 0x00],
            header_area_length: (4 * number_of_columns as u32 + 5).to_le_bytes(),
            version: [0x01, 0x00],
            filler: 0x00,
            number_of_columns: number_of_columns.to_le_bytes(),
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
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_file_header() {
        assert_eq!(vec!(78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, 05, 0, 0, 0, 1, 0, 0, 000, 0), Vec::from(FileHeader::new(0)));
        assert_eq!(vec!(78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, 09, 0, 0, 0, 1, 0, 0, 001, 0), Vec::from(FileHeader::new(1)));
        assert_eq!(vec!(78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, 13, 0, 0, 0, 1, 0, 0, 002, 0), Vec::from(FileHeader::new(2)));

        assert_eq!(vec!(78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, 01, 4, 0, 0, 1, 0, 0, 255, 0), Vec::from(FileHeader::new(255)));
        assert_eq!(vec!(78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, 05, 4, 0, 0, 1, 0, 0, 000, 1), Vec::from(FileHeader::new(256)));
        assert_eq!(vec!(78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, 09, 4, 0, 0, 1, 0, 0, 001, 1), Vec::from(FileHeader::new(257)));
    }

    #[test]
    fn enum_stuff() {
        println!("{:X}", ColumnTypes::Binary.as_bytes());
        println!("{:X}", ColumnTypes::VarBinary.as_bytes());
        println!("{:X}", ColumnTypes::Char(6).as_bytes());
    }


}
