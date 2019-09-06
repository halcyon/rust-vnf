use std::fmt;
use std::io::Write;


/// Function inputs will be column types in database ordinal order -> vnf header in byte array format.

/// Function inputs will be column types in database ordinal order -> iterator that will return bytes for vnf header

const SIGNATURE: [u8; 11] = [0x4E, 0x41, 0x54, 0x49, 0x56, 0x45, 0x0A, 0xFF, 0x0D, 0x0A, 0x00];
const HEADER_AREA_LENGTH: [u8; 4] = [0x3D, 0x0, 0x0, 0x0];
const VERSION: [u8; 2] = [0x01, 0x00];
const FILLER: u8 = 0x00;


// #[derive(Debug)]
struct Header {
    signature: [u8; 11],
    header_area_length: [u8; 4],
    version: [u8; 2],
    filler: u8,
    number_of_columns: [u8; 2],
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "signature: {:X?}", self.signature)?;
        writeln!(f, "header_area_length: {:X?}", self.header_area_length)?;
        writeln!(f, "version: {:X?}", self.version)?;
        writeln!(f, "filler: {:X?}", self.filler)?;
        writeln!(f, "number_of_columns: {:X?}", self.number_of_columns)
    }
}

impl Header {
    pub fn into_vec(&self) -> Vec<u8> {
        let mut sample: Vec<u8> = Vec::new();
        sample.extend(self.signature.iter());
        sample.extend(self.header_area_length.iter());
        sample.extend(self.version.iter());
        sample.push(self.filler);
        sample.extend(self.number_of_columns.iter().cloned());
        sample
    }
}

#[allow(dead_code)]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_hello() {
        let mut sample: Vec<u8> = Vec::new();
        let header = Header { signature: SIGNATURE,
                              header_area_length: HEADER_AREA_LENGTH,
                              version: VERSION,
                              filler: FILLER,
                              number_of_columns: [0x0E, 0x00],
        };
        sample.write(header.into_vec().as_slice());
        println!("{:0X?}", sample);

    }
}
