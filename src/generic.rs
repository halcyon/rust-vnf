use crate::column::{Type};
use std::io::{Write};




trait Append {
    fn append(&self, out: &mut dyn Write, column_type: &Type) -> std::io::Result<usize>;
}

impl Append for u64 {
    fn append(&self, out: &mut dyn Write, column_type: &Type) -> std::io::Result<usize> {
        match column_type {
            Type::Integer => out.write(&self.to_le_bytes()),
            _ => unimplemented!("ted")
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_append() {
        let mut out: Cursor<Vec<u8>> = Cursor::new(vec![]);

        5.append(& mut out, &Type::Integer).unwrap();

        println!("{:?}", &out);
    }
}
