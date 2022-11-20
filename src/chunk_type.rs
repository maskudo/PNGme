use core::fmt;
use std::fmt::Display;
use std::str;
use std::str::FromStr;

#[derive(Debug, PartialEq,Eq, Clone)]
pub struct ChunkType {
    critical: u8,
    public: u8,
    reserved: u8,
    safe_to_copy: u8,
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        [self.critical, self.public, self.reserved, self.safe_to_copy]
    }
    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
    fn is_critical(&self) -> bool {
        is_uppercase(&self.critical)
    }
    fn is_public(&self) -> bool {
        is_uppercase(&self.public)
    }
    fn is_reserved_bit_valid(&self) -> bool {
        is_uppercase(&self.reserved)
    }
    fn is_safe_to_copy(&self) -> bool {
        is_lowercase(&self.safe_to_copy)
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;
    fn try_from(list: [u8; 4]) -> Result<Self, Self::Error> {
        for byte in list.iter() {
            if !(is_lowercase(byte) || is_uppercase(byte)) {
                println!("{:?}", byte);
                return Err("invalid chunk type");
            }
        }
        Ok(ChunkType {
            critical: list[0],
            public: list[1],
            reserved: list[2],
            safe_to_copy: (list[3]),
        })
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        for byte in bytes {
            if !(is_lowercase(byte) || is_uppercase(byte)) {
                return Err("invalid chunk type");
            }
        }
        Ok(ChunkType {
            critical: bytes[0],
            public: bytes[1],
            reserved: bytes[2],
            safe_to_copy: bytes[3],
        })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = str::from_utf8(&[self.critical, self.public, self.reserved, self.safe_to_copy])
            .unwrap()
            .to_string();
        write!(f, "{}", s)
    }
}

fn is_uppercase(byte: &u8) -> bool {
    if byte > &64u8 && byte < &91u8 {
        return true;
    }
    false
}
fn is_lowercase(byte: &u8) -> bool {
    if byte > &96u8 && byte < &123u8 {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
