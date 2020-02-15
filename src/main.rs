use std::convert::From;
use std::fmt;
use std::u32;

use std::fs::File;
use std::io::prelude::*;

pub const SIGNATURE: [u8; 11] = [78, 65, 84, 73, 86, 69, 10, 255, 13, 10, 0];
pub const VERSION: [u8; 2] = [1, 0];
pub const FILLER: u8 = 0;

#[derive(Clone, Debug)]
enum ColumnType {
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
    //TODO: Numeric {precision: i32, scale: i32},
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
        }
    }
}

// struct Row<'a> {
//     columns: &'a [ColumnType],
//     data: &'a [&'a [u8]],
// }

// impl<'a> fmt::Display for Row<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[")?;
//         self.columns.iter().zip(self.data).for_each(|(col, data)| {
//             write!(f, "{:?} ", u32::from(col));
//             write!(f, "{:?},", data);
//         });
//         write!(f, "]")
//     }
// }

struct FileHeader {
    signature: [u8; 11],
    header_area_length: [u8; 4],
    version: [u8; 2],
    filler: u8,
    number_of_columns: [u8; 2],
    column_widths: Vec<u32>,
}

struct VNF {
    signature: [u8; 11],
    header_area_length: [u8; 4],
    version: [u8; 2],
    filler: u8,
    number_of_columns: [u8; 2],
    column_widths: Vec<u32>,
}

impl FileHeader {
    pub fn new(columns: Vec<ColumnType>) -> FileHeader {
        FileHeader {
            signature: SIGNATURE,
            header_area_length: ((4 * columns.len() + 5) as u32).to_le_bytes(),
            version: VERSION,
            filler: FILLER,
            number_of_columns: (columns.len() as u16).to_le_bytes(),
            column_widths: columns.into_iter().map(|col| u32::from(&col)).collect(),
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
        header
            .column_widths
            .iter()
            .for_each(|w| vec.extend(&w.to_le_bytes()));
        vec
    }
}

fn header_widths(columns: &[ColumnType]) -> Result<(), std::io::Error> {
    let mut file = File::create("foo.txt")?;
    let bytes = columns.iter()
                       .flat_map(|column| u32::from(column).to_le_bytes().to_vec())
                       .collect::<Vec<u8>>();
    println!("{:?}", bytes);
    file.write_all(bytes.as_slice())
}

fn main() -> Result<(), std::io::Error> {
    let columns = [
        ColumnType::Boolean,
        ColumnType::Integer,
        ColumnType::Integer,
        ColumnType::Integer,
    ];
    header_widths(&columns)?;
    Ok(())
}

// fn bytes(columns: &[ColumnType], data: &[&[u8]]) -> Vec<u8> {
//     // let bytes_in_row = columns.iter().fold(0, |acc,column_type| acc + u32::from(column_type) as usize);
//     // let mut buffer: Vec<u32> = vec![0; bytes_in_row];
//     // println!("buffer-length: {}", buffer.len());
//     // columns
//     //     .iter()
//     //     .zip(data)
//     //     .for_each(|(_x, y)| println!("{:X?}", y));

//     columns.iter().zip(data).fold(Vec::new(), |mut acc, (col_type, val)| {
//         // let padding = (u32::from(col_type) as usize) - val.len();
//         // println!("padding: {}", padding);
//         let mut buf: Vec<u8> = vec![0u8; u32::from(col_type) as usize];

//         let buf2 = val.iter().map(|e| e.to_le_bytes()).collect::<Vec<_>>();

//         // buf.as_bytes().to_le_bytes();
//         // buf.extend(y.iter());
//         // acc.iter().for_each(|x| println!("stuff: {:?}", x));
//         // let acc_mut: &mut Vec<u8> = acc;
//         // acc.extend_from_slice(&buf);
//         acc.extend_from_slice(buf2.as_slice());
//         acc
//     })

// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn new_file_header_with_no_columns() {
    //     let mut expected: Vec<u8> = Vec::new();
    //     let header_area_length: [u8; 4] = [5, 0, 0, 0];
    //     let number_of_columns: [u8; 2] = [0, 0];
    //     expected.extend(&SIGNATURE);
    //     expected.extend(&header_area_length);
    //     expected.extend(&VERSION);
    //     expected.push(FILLER);
    //     expected.extend(&number_of_columns);
    //     assert_eq!(expected, Vec::from(FileHeader::new(vec!())));
    // }

    // #[test]
    // fn new_file_header_with_one_column() {
    //     let mut expected: Vec<u8> = Vec::new();
    //     let header_area_length: [u8; 4] = [9, 0, 0, 0];
    //     let number_of_columns: [u8; 2] = [1, 0];
    //     let column_widths: Vec<u8> = vec![u8::MAX; 4];
    //     expected.extend(&SIGNATURE);
    //     expected.extend(&header_area_length);
    //     expected.extend(&VERSION);
    //     expected.push(FILLER);
    //     expected.extend(&number_of_columns);
    //     expected.extend(column_widths);
    //     assert_eq!(
    //         expected,
    //         Vec::from(FileHeader::new(vec!(ColumnType::VarChar)))
    //     );
    // }

    // #[test]
    // fn new_file_header_with_two_columns() {
    //     let mut expected: Vec<u8> = Vec::new();
    //     let header_area_length: [u8; 4] = [13, 0, 0, 0];
    //     let number_of_columns: [u8; 2] = [2, 0];
    //     let column_widths: Vec<u8> = vec![u8::MAX, u8::MAX, u8::MAX, u8::MAX, 4, 0, 0, 0];
    //     expected.extend(&SIGNATURE);
    //     expected.extend(&header_area_length);
    //     expected.extend(&VERSION);
    //     expected.push(FILLER);
    //     expected.extend(&number_of_columns);
    //     expected.extend(column_widths);
    //     assert_eq!(
    //         expected,
    //         Vec::from(FileHeader::new(vec!(
    //             ColumnType::VarChar,
    //             ColumnType::Char(4)
    //         )))
    //     );
    // }

    // #[test]
    // fn new_file_header_with_255_columns() {
    //     let mut expected: Vec<u8> = Vec::new();
    //     let header_area_length: [u8; 4] = [1, 4, 0, 0];
    //     let number_of_columns: [u8; 2] = [255, 0];
    //     let column_widths: Vec<u8> = vec![u8::MAX; 1020];
    //     expected.extend(&SIGNATURE);
    //     expected.extend(&header_area_length);
    //     expected.extend(&VERSION);
    //     expected.push(FILLER);
    //     expected.extend(&number_of_columns);
    //     expected.extend(column_widths);
    //     assert_eq!(
    //         expected,
    //         Vec::from(FileHeader::new(vec!(ColumnType::VarBinary; 255)))
    //     );
    // }

    // #[test]
    // fn new_file_header_with_256_columns() {
    //     let mut expected: Vec<u8> = Vec::new();
    //     let header_area_length: [u8; 4] = [5, 4, 0, 0];
    //     let number_of_columns: [u8; 2] = [0, 1];
    //     let column_widths: Vec<u8> = vec![u8::MAX; 1024];
    //     expected.extend(&SIGNATURE);
    //     expected.extend(&header_area_length);
    //     expected.extend(&VERSION);
    //     expected.push(FILLER);
    //     expected.extend(&number_of_columns);
    //     expected.extend(column_widths);
    //     assert_eq!(
    //         expected,
    //         Vec::from(FileHeader::new(vec!(ColumnType::VarBinary; 256)))
    //     );
    // }

    // #[test]
    // fn new_file_header_with_257_columns() {
    //     let mut expected: Vec<u8> = Vec::new();
    //     let header_area_length: [u8; 4] = [9, 4, 0, 0];
    //     let number_of_columns: [u8; 2] = [1, 1];
    //     let column_widths: Vec<u8> = vec![u8::MAX; 1028];
    //     expected.extend(&SIGNATURE);
    //     expected.extend(&header_area_length);
    //     expected.extend(&VERSION);
    //     expected.push(FILLER);
    //     expected.extend(&number_of_columns);
    //     expected.extend(column_widths);
    //     assert_eq!(
    //         expected,
    //         Vec::from(FileHeader::new(vec!(ColumnType::VarBinary; 257)))
    //     );
    // }

    // #[test]
    // fn u32_from_column_types() {
    //     assert_eq!(1, u32::from(&ColumnType::Boolean));
    //     assert_eq!(3, u32::from(&ColumnType::Binary(3)));
    //     assert_eq!(8, u32::from(&ColumnType::Integer));
    //     assert_eq!(8, u32::from(&ColumnType::Interval));
    //     assert_eq!(8, u32::from(&ColumnType::Time));
    //     assert_eq!(14, u32::from(&ColumnType::Char(14)));
    //     assert_eq!(u32::MAX, u32::from(&ColumnType::VarBinary));
    //     assert_eq!(u32::MAX, u32::from(&ColumnType::VarChar));
    // }

    #[test]
    // fn test_row() {
    //     let row = Row {
    //         columns: &[
    //             ColumnType::Boolean,
    //             ColumnType::Integer,
    //             ColumnType::Integer,
    //             ColumnType::Integer,
    //         ], // data: &[&[u8::from(true)], &[12u8], &[1u8], &[2u8]]
    //     };
    //     println!("Row: {}", row);
    // }
    fn test_widths() {
        // let mut file = File::create("foo.txt")?;
        let columns = [
            ColumnType::Boolean,
            ColumnType::Integer,
            ColumnType::Float,
            ColumnType::Char(6),
            ColumnType::VarChar
        ];
        columns.iter().for_each(|col| {
            // println!("{}", u32::from(col).to_le_bytes());
            let mut bytes: [u8; 4] = u32::from(col).to_le_bytes();
            bytes.iter().for_each(|byte| println!("{}", byte));
            // let mut bytes = u32::from(col).to_le_bytes();
            // bytes.iter().for_each(|byte| file.write_all(byte));
        });
    }

    // fn test_bytes() {
    //     bytes(
    //         &[  ColumnType::Boolean,
    //             ColumnType::Integer,
    //             ColumnType::Integer,
    //             ColumnType::Integer,
    //         ],
    //         &[&[u8::from(true)], &[12u8], &[1u8], &[2u8], // "abcd".as_bytes()
    //         ],
    //     ).iter().for_each(|x| print!("{:02X}, ", x));
    //     // println!("{:02X?}", bytes(
    //     //     &[
    //     //         ColumnType::Integer,
    //     //         ColumnType::Integer,
    //     //         ColumnType::Integer,
    //     //         ColumnType::VarChar,
    //     //     ],
    //     //     &[&[12u8], &[1u8], &[2u8], "abcd".as_bytes()],
    //     // ).as_slice());
    // }
}
