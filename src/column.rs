use std::u32;

#[derive(Copy, Clone, Debug)]
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

pub fn numeric_width(precision: u32) -> u32 {
    ((precision / 19) + 1) * 8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_width() {
        assert_eq!(24, numeric_width(38));
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
}
