/// Function inputs will be column types in database ordinal order -> vnf header in byte array format.

/// Function inputs will be column types in database ordinal order -> iterator that will return bytes for vnf header

#[allow(dead_code)]

static SIGNATURE: [u8; 11] = [0x4E, 0x41, 0x54, 0x49, 0x56, 0x45, 0x0A, 0xFF, 0x0D, 0x0A, 0x00];

fn header() -> [u8; 11] {
    SIGNATURE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn another_test() {
        assert_eq!(SIGNATURE, header());
    }
}
