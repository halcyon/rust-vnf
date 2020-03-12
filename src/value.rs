use std::io;

pub enum Value {
    Null,
    Integer(i64),
    Boolean(bool),
}

use Value::*;

impl Value {
    pub fn write<'a, W: io::Write>(&self, out: &'a mut W) -> io::Result<usize> {
        match self {
            Null => Ok(0),
            Integer(value) => out.write(&value.to_le_bytes()),
            Boolean(value) => out.write(&[if *value { 1u8 } else { 0u8 }]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null() {
        let mut out: Vec<u8> = vec![];
        Null.write(&mut out);
        assert_eq!(0, out.len());
    }

    #[test]
    fn integer() {
        let mut out: Vec<u8> = vec![];
        Integer(1).write(&mut out);
        assert_eq!(vec![1u8, 0, 0, 0, 0, 0, 0, 0], out);
    }

    fn boolean() {
        let mut out: Vec<u8> = vec![];
        Boolean(true).write(&mut out);
        assert_eq!(vec![1u8], out);
    }
}
