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
    use crate::column::ColumnType;
    use crate::date::VerticaDate;
    use crate::file_header::{FileHeader, FILLER, SIGNATURE, VERSION};
    use chrono::NaiveDate;

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
            NaiveDate::from_ymd(1999, 1, 8)
                .to_y2k_epoch_duration()
                .num_days()
                .to_le_bytes()
                .to_vec(),
        );
        data.extend(
            NaiveDate::from_ymd(1999, 2, 23)
                .and_hms_micro(3, 11, 52, 350_000)
                .to_y2k_epoch_duration()
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
        assert_eq!(
            NaiveDate::from_ymd(1999, 1, 8)
                .to_y2k_epoch_duration()
                .num_days()
                .to_le_bytes(),
            &example[116..124]
        ); // Date - 1999-01-08

        assert_eq!(
            NaiveDate::from_ymd(1999, 2, 23)
                .and_hms_micro(3, 11, 52, 350_000)
                .to_y2k_epoch_duration()
                .num_microseconds()
                .unwrap()
                .to_le_bytes(),
            &example[124..132]
        ); // TIMESTAMP - 1999-02-23 03:11:52.35
    }
}
