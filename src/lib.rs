use std::convert::From;
use std::fmt;
use std::u32;
use std::u8;

use std::fs::File;
use std::io::Write;

use chrono::{NaiveDate, Duration};

pub const SIGNATURE: [u8; 11] = [78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0];
pub const VERSION: [u8; 2] = [1, 0];
pub const FILLER: u8 = 0;

#[derive(Clone, Debug)]
pub enum ColumnType {
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
    Binary(u32),
    Numeric { precision: u32, _scale: u32 },
    Interval,
}

impl From<&ColumnType> for u32 {
    fn from(column: &ColumnType) -> Self {
        match *column {
            ColumnType::Boolean => 1,

            ColumnType::Integer
            | ColumnType::Float
            | ColumnType::Date
            | ColumnType::Timestamp
            | ColumnType::TimestampTz
            | ColumnType::Time
            | ColumnType::TimeTz
            | ColumnType::Interval => 8,

            ColumnType::Char(length) | ColumnType::Binary(length) => length,

            ColumnType::VarChar | ColumnType::VarBinary => u32::MAX,

            ColumnType::Numeric { precision, _scale } => numeric_width(precision),
        }
    }
}

fn numeric_width(precision: u32) -> u32 {
    ((precision / 19) + 1) * 8
}

pub struct FileHeader {
    signature: [u8; 11],
    header_area_length: [u8; 4],
    version: [u8; 2],
    filler: u8,
    number_of_columns: [u8; 2],
    column_widths: Vec<u8>,
}

impl FileHeader {
    pub fn new(columns: Vec<ColumnType>) -> FileHeader {
        FileHeader {
            signature: SIGNATURE,
            header_area_length: ((4 * columns.len() + 5) as u32).to_le_bytes(),
            version: VERSION,
            filler: FILLER,
            number_of_columns: (columns.len() as u16).to_le_bytes(),
            column_widths: columns
                .as_slice()
                .iter()
                .flat_map(|column| u32::from(column).to_le_bytes().to_vec())
                .collect(),
        }
    }
}

impl fmt::Display for FileHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "signature: {:X?}", self.signature)?;
        writeln!(f, "header_area_length: {:X?}", self.header_area_length)?;
        writeln!(f, "version: {:X?}", self.version)?;
        writeln!(f, "filler: {:X?}", self.filler)?;
        writeln!(f, "number_of_columns: {:X?}", self.number_of_columns)?;
        writeln!(f, "column_widths: {:X?},", self.column_widths)
    }
}

impl From<FileHeader> for Vec<u8> {
    fn from(header: FileHeader) -> Self {
        let mut vec: Vec<u8> = Vec::new();
        vec.extend_from_slice(&header.signature);
        vec.extend_from_slice(&header.header_area_length);
        vec.extend_from_slice(&header.version);
        vec.push(header.filler);
        vec.extend_from_slice(&header.number_of_columns);
        vec.extend(&header.column_widths);
        vec
    }
}

fn _write_bytes(file_name: &str, bytes: &[u8]) -> Result<(), std::io::Error> {
    let mut file = File::create(file_name)?;
    file.write_all(bytes)
}

pub struct Row {
    data_length: u32,
    null_bit_field: Vec<u8>,
    data: Vec<u8>,
}

impl Row {
    pub fn new(null_bit_field: Vec<u8>, data: Vec<u8>) -> Row {
        Row {
            data_length: data.len() as u32,
            null_bit_field: null_bit_field,
            data: data,
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

pub struct VerticaDate {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    sec: u32,
}

impl VerticaDate {
    pub fn from_ymd(year: i32, month: u32, day: u32) -> VerticaDate {
        VerticaDate {
            year,
            month,
            day,
            hour: 0,
            min: 0,
            sec: 0,
        }
    }

    pub fn and_hms(self, hour: u32, min: u32, sec: u32) -> VerticaDate {
        VerticaDate {
            year: self.year,
            month: self.month,
            day: self.day,
            hour,
            min,
            sec
        }
    }

    pub fn num_days(self) -> i64 {
        self.duration().num_days()
    }

    pub fn num_microseconds(self) -> Option<i64> {
        self.duration().num_microseconds()
    }

    fn duration(self) -> Duration {
        (NaiveDate::from_ymd(self.year, self.month, self.day).and_hms(self.hour, self.min, self.sec) - NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0))
    }
}

// pub struct VerticaDateTime {
//     duration: Duration,
// }

// impl VerticaDateTime {
//     pub fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> VerticaDate {
//         VerticaDate {
//             duration: NaiveDate::from_ymd(year, month, day).and_hms(hour, minute, second) -
//                 NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_width() {
        assert_eq!(24, numeric_width(38));
    }

    #[test]
    fn test_vertica_epoch_days() {
        assert_eq!(-358, VerticaDate::from_ymd(1999, 1, 8).num_days());
        assert_eq!(0, VerticaDate::from_ymd(2000, 1, 1).num_days());
        assert_eq!(366, VerticaDate::from_ymd(2001, 1, 1).num_days());
    }

    #[test]
    fn new_file_header_with_no_columns() {
        assert_eq!(
            vec![
                78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0, // SIGNATURE
                5, 0, 0, 0, // header_area_length
                1, 0, // VERSION
                0, // FILLER
                0, 0 // number_of_columns
            ],
            Vec::from(FileHeader::new(vec!()))
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
            Vec::from(FileHeader::new(vec!(ColumnType::VarChar)))
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
            Vec::from(FileHeader::new(vec!(
                ColumnType::VarChar,
                ColumnType::Char(4),
            )))
        );
        println!(
            "{}",
            FileHeader::new(vec!(ColumnType::VarChar, ColumnType::Char(4),))
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
        expected.push(FILLER);
        expected.extend(&number_of_columns);
        expected.extend(column_widths);
        assert_eq!(
            expected,
            Vec::from(FileHeader::new(vec!(ColumnType::VarBinary; 255)))
        );
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
        expected.push(FILLER);
        expected.extend(&number_of_columns);
        expected.extend(column_widths);
        assert_eq!(
            expected,
            Vec::from(FileHeader::new(vec!(ColumnType::VarBinary; 256)))
        );
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
        expected.push(FILLER);
        expected.extend(&number_of_columns);
        expected.extend(column_widths);
        assert_eq!(
            expected,
            Vec::from(FileHeader::new(vec!(ColumnType::VarBinary; 257)))
        );
    }

    #[test]
    fn u32_from_column_types() {
        assert_eq!(1, u32::from(&ColumnType::Boolean));
        assert_eq!(3, u32::from(&ColumnType::Binary(3)));
        assert_eq!(8, u32::from(&ColumnType::Integer));
        assert_eq!(8, u32::from(&ColumnType::Interval));
        assert_eq!(8, u32::from(&ColumnType::Time));
        assert_eq!(14, u32::from(&ColumnType::Char(14)));
        assert_eq!(u32::MAX, u32::from(&ColumnType::VarBinary));
        assert_eq!(u32::MAX, u32::from(&ColumnType::VarChar));
    }

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

    #[test]
    fn example() {
        let _expected = vec![
            0x4E, 0x41, 0x54, 0x49, 0x56, 0x45, 0x0A, 0xFF, 0x0D, 0x0A, 0x00, 0x3D, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x0E, 0x00, 0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
            0x0A, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x00, 0x00, 0x00, 0x08, 0x00,
            0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
            0x08, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x03, 0x00, 0x00, 0x00, 0x18, 0x00,
            0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x73, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC3, 0xF5, 0x28, 0x5C, 0x8F, 0xC2, 0xF1, 0xBF,
            0x6F, 0x6E, 0x65, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x03, 0x00, 0x00, 0x00,
            0x4F, 0x4E, 0x45, 0x01, 0x9A, 0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x30, 0x85,
            0xB3, 0x4F, 0x7E, 0xE7, 0xFF, 0xFF, 0x40, 0x1F, 0x3E, 0x64, 0xE8, 0xE3, 0xFF, 0xFF,
            0xC0, 0x2E, 0x98, 0xFF, 0x05, 0x00, 0x00, 0x00, 0xD0, 0x97, 0x01, 0x80, 0xF0, 0x79,
            0xF0, 0x10, 0x02, 0x00, 0x00, 0x00, 0xAB, 0xCD, 0xAB, 0xCD, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x64,
            0xD6, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x47, 0xA3, 0x8E, 0x02, 0x00, 0x00,
            0x00,
        ];
        let cols = vec![
            ColumnType::Integer,
            ColumnType::Float,
            ColumnType::Char(10),
            ColumnType::VarChar,
            ColumnType::Boolean,
            ColumnType::Date,
            ColumnType::Timestamp,
            ColumnType::TimestampTz,
            ColumnType::Time,
            ColumnType::TimeTz,
            ColumnType::VarBinary,
            ColumnType::Binary(3),
            ColumnType::Numeric {
                precision: 38,
                _scale: 0,
            },
            ColumnType::Interval,
        ];
        let mut example = Vec::from(FileHeader::new(cols.clone()));
        let mut data = Vec::new();
        data.extend(1u64.to_le_bytes().to_vec());
        data.extend((-1.11f64).to_le_bytes().to_vec());
        data.extend("one       ".as_bytes());
        data.extend(("ONE".len() as u32).to_le_bytes().to_vec());
        data.extend("ONE".as_bytes());
        data.push(1);
        data.extend(
            VerticaDate::from_ymd(1999, 1, 8)
                .num_days()
                .to_le_bytes()
                .to_vec(),
        );
        data.extend(
            VerticaDate::from_ymd(1999, 2, 23)
                .and_hms(3, 11, 52.35)
                .num_microseconds()
                .unwrap()
                .to_le_bytes()
                .to_vec(),
        );

        let row = Vec::from(Row::new(vec![0, 0], data.clone()));
        example.extend(row);

        assert_eq!(&SIGNATURE, &example[0..11]);
        assert_eq!(
            ((4 * cols.len() + 5) as u32).to_le_bytes(),
            &example[11..15]
        );
        assert_eq!(&VERSION, &example[15..17]);
        assert_eq!(&FILLER, &example[17]);
        assert_eq!((cols.len() as u16).to_le_bytes(), &example[18..20]);
        assert_eq!(
            vec![
                8u8, 0, 0, 0, 8, 0, 0, 0, 10, 0, 0, 0, 255, 255, 255, 255, 1, 0, 0, 0, 8, 0, 0, 0,
                8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 255, 255, 255, 255, 3, 0, 0, 0, 24,
                0, 0, 0, 8, 0, 0, 0,
            ]
            .as_slice(),
            &example[20..76]
        );
        //TODO: row header - row length
        assert_eq!(0u16.to_le_bytes(), &example[80..82]); // Bit field
        assert_eq!(&[1u8, 0, 0, 0, 0, 0, 0, 0], &example[82..90]); // Integer
        assert_eq!((-1.11f64).to_le_bytes(), &example[90..98]); // Float
        assert_eq!("one       ".as_bytes(), &example[98..108]); // Char(10)
        assert_eq!(("ONE".len() as u32).to_le_bytes(), &example[108..112]); // Number of bytes in following VarChar
        assert_eq!("ONE".as_bytes(), &example[112..115]); // Var Char
        assert_eq!(&[1u8], &example[115..116]); // Boolean
        assert_eq!((-358i64).to_le_bytes(), &example[116..124]); // Date - 1999-01-08


        // assert_eq!(expected, example);
    }
}
