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
    use crate::file_header::{FILLER, SIGNATURE, VERSION};
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
        let expected = vec![
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

        assert_eq!(&expected[0..11], &SIGNATURE);
        assert_eq!(
            expected[11..15],
            ((4 * cols.len() + 5) as u32).to_le_bytes()
        );
        assert_eq!(&expected[15..17], &VERSION);
        assert_eq!(&expected[17], &FILLER);
        assert_eq!(expected[18..20], (cols.len() as u16).to_le_bytes());
        assert_eq!(
            &expected[20..76],
            vec![
                8u8, 0, 0, 0, 8, 0, 0, 0, 10, 0, 0, 0, 255, 255, 255, 255, 1, 0, 0, 0, 8, 0, 0, 0,
                8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 255, 255, 255, 255, 3, 0, 0, 0, 24,
                0, 0, 0, 8, 0, 0, 0,
            ]
            .as_slice()
        );
        //TODO: row header - row length
        assert_eq!(expected[80..82], 0u16.to_le_bytes()); // Bit field
        assert_eq!(&expected[82..90], &[1u8, 0, 0, 0, 0, 0, 0, 0]); // Integer
        assert_eq!(expected[90..98], (-1.11f64).to_le_bytes()); // Float
        assert_eq!(&expected[98..108], "one       ".as_bytes()); // Char(10)
        assert_eq!(expected[108..112], ("ONE".len() as u32).to_le_bytes()); // Number of bytes in following VarChar
        assert_eq!(&expected[112..115], "ONE".as_bytes()); // Var Char
        assert_eq!(&expected[115..116], &[1u8]); // Boolean
        assert_eq!(
            &expected[116..124],
            NaiveDate::from_ymd(1999, 1, 8)
                .to_y2k_epoch_duration()
                .num_days()
                .to_le_bytes()
        ); // Date - 1999-01-08

        assert_eq!(
            &expected[124..132],
            NaiveDate::from_ymd(1999, 2, 23)
                .and_hms_micro(3, 11, 52, 350_000)
                .to_y2k_epoch_duration()
                .num_microseconds()
                .unwrap()
                .to_le_bytes()
        ); // TIMESTAMP - 1999-02-23 03:11:52.35

        assert_eq!(
            &expected[132..140],
            NaiveDate::from_ymd(1999, 1, 8)
                .and_hms(12, 4, 37)
                .to_y2k_epoch_duration()
                .num_microseconds()
                .unwrap()
                .to_le_bytes()
        ); // TIMESTAMPTZ - 1999-01-08 07:04:37-05
    }
}
