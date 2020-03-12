#![allow(unused)]

use std::u32;

#[derive(Copy, Clone, Debug)]
pub enum Type {
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

impl From<&Type> for u32 {
    fn from(column: &Type) -> Self {
        match *column {
            Type::Boolean => 1,

            Type::Integer
            | Type::Float
            | Type::Date
            | Type::Timestamp
            | Type::TimestampTz
            | Type::Time
            | Type::TimeTz
            | Type::Interval => 8,

            Type::Char(length) | Type::Binary(length) => length,

            Type::VarChar | Type::VarBinary => u32::MAX,

            Type::Numeric { precision, _scale } => numeric_width(precision),
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
        assert_eq!(1, u32::from(&Type::Boolean));
        assert_eq!(3, u32::from(&Type::Binary(3)));
        assert_eq!(8, u32::from(&Type::Integer));
        assert_eq!(8, u32::from(&Type::Interval));
        assert_eq!(8, u32::from(&Type::Time));
        assert_eq!(14, u32::from(&Type::Char(14)));
        assert_eq!(u32::MAX, u32::from(&Type::VarBinary));
        assert_eq!(u32::MAX, u32::from(&Type::VarChar));
    }
}
