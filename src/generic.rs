use crate::column::{Type};
use std::io::{Write};




trait Append<T> {
    fn append(out: &mut dyn Write, column_type: &Type, input: &T) -> std::io::Result<usize>;
}

impl<T> Append<T> for u64 {
    fn append(out: &mut dyn Write, column_type: &Type, input: &u64) -> std::io::Result<usize> {
        match column_type {
            Type::Integer => out.write(input.to_le_bytes()),
            _ => unimplemented!("ted")

        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append() {
        let mut out = Vec::new();

        append(out, &Column::Type::Int, 5);

        println!(out);



    }
}
