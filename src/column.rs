use std::u32;
use crate::errors::*;

#[derive(Copy, Clone, Debug)]
pub enum Type {
    Boolean,
    Integer,
    Float,
    Char { len: usize },
    VarChar,
    Date,
    Timestamp,
    TimestampTz,
    Time,
    TimeTz,
    VarBinary,
    Binary { len: usize },
    Numeric { precision: u32, _scale: u32 },
    Interval,
}

impl From<&Type> for u32 {
    fn from(column: &Type) -> Self {
        type_to_length(column)
    }
}

fn type_to_length(column: &Type) -> u32 {
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

        Type::Char { len } | Type::Binary { len } => len as u32,

        Type::VarChar | Type::VarBinary => u32::MAX,

        Type::Numeric { precision, _scale } => numeric_width(precision),
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Value<'a> {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Char(&'a str),
}

impl Type {
    pub fn append(&self, buffer: &mut Vec<u8>, value: &Value) {
        match (self, value) {
            (_, Value::Null) => return,
            (Type::Boolean, Value::Boolean(b)) =>
                buffer.push(if *b { 1u8 } else { 0u8 }),
            (Type::Integer, Value::Integer(i)) =>
                buffer.extend_from_slice(&i.to_le_bytes()),
            (Type::Float, Value::Float(f)) =>
                buffer.extend_from_slice(&f.to_bits().to_le_bytes()),
            (Type::Char{len}, Value::Char(s)) => {
                let char_len = std::cmp::min(*len, s.len());
                let pad_len = if *len > s.len() { *len - s.len() } else { 0 };
                buffer.extend_from_slice(&s.as_bytes()[0..char_len]);
                for _ in 0..pad_len {
                    buffer.push(0x20);
                }
            }
            (_, value) => unimplemented!("({:?}, {:?})", self, value),
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
        assert_eq!(3, u32::from(&Type::Binary { len: 3 }));
        assert_eq!(8, u32::from(&Type::Integer));
        assert_eq!(8, u32::from(&Type::Interval));
        assert_eq!(8, u32::from(&Type::Time));
        assert_eq!(14, u32::from(&Type::Char { len: 14 }));
        assert_eq!(u32::MAX, u32::from(&Type::VarBinary));
        assert_eq!(u32::MAX, u32::from(&Type::VarChar));
    }

    #[test]
    fn boolean() {
        let mut out: Vec<u8> = vec![];
        Type::Boolean.append(&mut out, &Value::Boolean(true));
        Type::Boolean.append(&mut out, &Value::Boolean(false));
        assert_eq!(vec![1u8, 0u8], out);
    }

    #[test]
    fn integer() {
        let mut out: Vec<u8> = vec![];
        Type::Integer.append(&mut out, &Value::Integer(1));
        assert_eq!(vec![1u8, 0, 0, 0, 0, 0, 0, 0], out);
    }

    #[test]
    fn float() {
        let mut out: Vec<u8> = vec![];
        Type::Float.append(&mut out, &Value::Float(-1.11));
        assert_eq!(vec![0xc3u8, 0xf5, 0x28, 0x5c, 0x8f, 0xc2, 0xf1, 0xbf], out);
    }

    #[test]
    fn char_from_str() {
        let mut out: Vec<u8> = vec![];
        Type::Char { len: 3 }.append(&mut out, &Value::Char("ABC"));
        assert_eq!(vec![0x41, 0x42, 0x43], out);
    }

    #[test]
    fn char_underflow() {
        let mut out: Vec<u8> = vec![];
        Type::Char { len: 3 }.append(&mut out, &Value::Char("AB"));
        assert_eq!(vec![0x41, 0x42, 0x20], out);
    }

    #[test]
    fn char_overflow() {
        let mut out: Vec<u8> = vec![];
        Type::Char { len: 3 }.append(&mut out, &Value::Char("ABCD"));
        assert_eq!(vec![0x41, 0x42, 0x43], out);
    }
}
