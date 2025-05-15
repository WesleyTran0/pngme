use crate::conversions::*;
use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

/// Represents a ChunkType stored in every Chunk
#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    // arrays stored in each field are MSBit in idx 0 and LSBit in idx 7
    // byte_four represents the MSByte and byte_one represents the LSByte
    byte_four: [bool; 8],
    byte_three: [bool; 8],
    byte_two: [bool; 8],
    byte_one: [bool; 8],
}

/// Represents an error encountered when parsing an input for ChunkType
#[derive(Debug, PartialEq, Eq)]
pub struct ParseChunkTypeError;

/// Allows the ChunkType to be made from an array of 4 u8 integers
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ParseChunkTypeError;

    fn try_from(nums: [u8; 4]) -> Result<Self, Self::Error> {
        if nums.len() == 4 {
            let mut bytes = [[false; 8]; 4];

            // EFFECT: Converts the u8 integers to an array of their bits idx 0 is the
            //  MSBit and the left most integer idx 3 is the LSBit and the right most
            for idx in 0..4 {
                let cur_num = nums[idx];

                if (cur_num < 65) || (cur_num > 90 && cur_num < 97) || (cur_num > 122) {
                    return Err(ParseChunkTypeError);
                }

                bytes[idx] = u8_to_bits(cur_num);
            }

            Ok(ChunkType::from_arr_bytes(bytes))
        } else {
            Err(ParseChunkTypeError)
        }
    }
}

/// Allows a ChunkType to be made from a 4 character String. Must be alphabetic characters
impl FromStr for ChunkType {
    // arrays stored in each field are MSBit in idx 0 and LSBit in idx 7
    // byte_four represents the MSByte and byte_one represents the LSByte

    type Err = ParseChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            Err(ParseChunkTypeError)
        } else {
            let temp_str = String::from(s);
            let temp_chars = temp_str.chars().enumerate();
            let mut bytes: [u8; 4] = [0, 0, 0, 0];

            // EFFECT: Adds each string as an u8 to bytes
            for (idx, my_char) in temp_chars {
                bytes[idx] = char_to_u8(my_char);
            }
            ChunkType::try_from(bytes)
        }
    }
}

/// Allows a ChunkType to be displayed as a string
impl Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ct_str = String::new();
        let bytes = &self.bytes();

        let ct_str = ct_str
            + &u8_to_string(bytes[0])
            + &u8_to_string(bytes[1])
            + &u8_to_string(bytes[2])
            + &u8_to_string(bytes[3]);

        write!(f, "{ct_str}")
    }
}

#[allow(dead_code)]
/// Represents a ChunkType consisting of 4 bytes. Byte_four is the left and MSByte
/// The bits in each byte are ordered from MSBit to LSBit
impl ChunkType {
    /// Creates a ChunkType from 4 bit representations of bytes. Where index0 is the MSB
    fn from_arr_bytes(bytes: [[bool; 8]; 4]) -> ChunkType {
        ChunkType {
            byte_four: bytes[0],
            byte_three: bytes[1],
            byte_two: bytes[2],
            byte_one: bytes[3],
        }
    }
    /// Converts this ChunkType's bits to their byte representation
    pub fn bytes(&self) -> [u8; 4] {
        [
            bits_to_byte(&self.byte_four),
            bits_to_byte(&self.byte_three),
            bits_to_byte(&self.byte_two),
            bits_to_byte(&self.byte_one),
        ]
    }

    /// Determines if this ChunkType is valid; A ChunkType is valid if the
    /// 2nd byte is an uppercase letter
    fn is_valid(&self) -> bool {
        let byte_three_num = bits_to_byte(&self.byte_two);

        if (byte_three_num < 65)
            || (byte_three_num > 90 && byte_three_num < 97)
            || (byte_three_num > 122)
        {
            panic!("Invalid number given. Does not map to an ASCII letter");
        }

        !&self.byte_two[2]
    }

    /// Determines if this ChunkType is critical (false) or ancillary (true)
    /// A ChunkType is one or the other based on if the 4th byte is an uppercase letter
    fn is_critical(&self) -> bool {
        !&self.byte_four[2]
    }

    /// Determines if this ChunkType is public.
    /// A ChunkType is public if the 3rd byte is a lowercase letter
    fn is_public(&self) -> bool {
        !&self.byte_three[2]
    }

    /// Determines if this ChunkType is reserved.
    /// A ChunkType is reserved if it is valid
    fn is_reserved_bit_valid(&self) -> bool {
        *&self.is_valid()
    }

    /// Determines if this ChunkType is safe-to-copy.
    /// A ChunkType is safe-to-copy if the 1st byte is a lowercase letter
    fn is_safe_to_copy(&self) -> bool {
        *&self.byte_one[2]
    }
}

#[allow(unused_variables)]
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
