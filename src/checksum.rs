use adler32::RollingAdler32;

pub fn adler32_from_bytes(bytes: &[u8; 4]) -> u32 {
    (u32::from(bytes[3]))
        | (u32::from(bytes[2]) << 8)
        | (u32::from(bytes[1]) << 16)
        | (u32::from(bytes[0]) << 24)
}

/// Whether we should validate the checksum, and what type of checksum it is.
pub enum ChecksumType {
    /// No validation.
    ///
    /// For raw deflate streams or when we don't bother checking.
    None,
    /// Adler32
    ///
    /// Used in the zlib format.
    Adler32(RollingAdler32),
}

pub struct Checksum {
    checksum_type: ChecksumType,
}

impl Checksum {
    #[inline]
    pub fn none() -> Checksum {
        Checksum::new(ChecksumType::None)
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        match self.checksum_type {
            ChecksumType::None => true,
            _ => false,
        }
    }

    #[inline]
    pub fn zlib() -> Checksum {
        Checksum::new(ChecksumType::Adler32(RollingAdler32::new()))
    }

    pub fn new(checksum_type: ChecksumType) -> Checksum {
        Checksum { checksum_type }
    }

    #[inline]
    pub fn update(&mut self, bytes: &[u8]) {
        match self.checksum_type {
            ChecksumType::None => (),
            ChecksumType::Adler32(ref mut c) => {
                c.update_buffer(bytes);
            }
        }
    }

    pub fn check(&self, expected: u32) -> Result<(), String> {
        match self.checksum_type {
            ChecksumType::None => Ok(()),
            ChecksumType::Adler32(ref c) => {
                if c.hash() == expected {
                    Ok(())
                } else {
                    Err("Checksum mismatch!".to_owned())
                }
            }
        }
    }

    #[inline]
    pub fn current_value(&self) -> u32 {
        match self.checksum_type {
            ChecksumType::Adler32(ref c) => c.hash(),
            _ => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::adler32_from_bytes;

    #[test]
    fn adler32() {
        let bytes = [0x00, 0x00, 0x01, 0x0b];
        assert_eq!(adler32_from_bytes(&bytes), 267);
    }
}
