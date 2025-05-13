use crate::{
    chunk_type::ChunkType,
    conversions::{bytes_to_u32, u8_to_string, u32_to_bytes},
};
use crc::{CRC_32_ISO_HDLC, Crc};
use std::fmt;
use std::fmt::{Debug, Display};

// Represents a Chunk of an image
pub struct Chunk {
    // length and crc are both 4 byte unsigned integers
    length: u32,
    chunk_type: ChunkType,
    chunk_data_bytes: Vec<u8>,
    crc: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseChunkError;

// Allows this Chunk to be made from a vec of bytes where:
// the first 4 bytes are length, next 4 are the ChunkType, the last 4 are the crc
// and the other bytes are the message in the chunk
impl TryFrom<&Vec<u8>> for Chunk {
    type Error = ParseChunkError;

    fn try_from(data: &Vec<u8>) -> Result<Self, Self::Error> {
        if data.len() < 12 {
            return Err(ParseChunkError);
        }
        let length_bytes = [data[0], data[1], data[2], data[3]];
        let length = bytes_to_u32(length_bytes);

        let data_end_idx = data.len() - 4;
        let chunk_data_bytes = data[8..data_end_idx].to_vec();

        if chunk_data_bytes.len() as u32 != length {
            return Err(ParseChunkError);
        }

        let chunk_type = ChunkType::try_from([data[4], data[5], data[6], data[7]]).unwrap();

        let calculated_crc = Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(&data[4..data_end_idx]);
        let crc = bytes_to_u32([
            data[data_end_idx],
            data[data_end_idx + 1],
            data[data_end_idx + 2],
            data[data_end_idx + 3],
        ]);

        if calculated_crc != crc {
            return Err(ParseChunkError);
        }

        Ok(Chunk {
            length,
            chunk_type,
            chunk_data_bytes,
            crc,
        })
    }
}

// Allows this Chunk to be display in a string through formatting
impl Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.data_as_string() {
            Ok(str) => write!(f, "{}", str),
            Err(_) => write!(f, "There was an error displaying chunk data"),
        }
    }
}

// independent functions for Chunk
impl Chunk {
    // Creates a new Chunk object from the given ChunkType and data as bytes
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length = data.len() as u32;

        let chunk_type_bytes = chunk_type.bytes();
        let mut combined = data.clone();

        combined.splice(0..0, chunk_type_bytes.iter().cloned());

        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(&combined[0..combined.len()]);

        Chunk {
            length,
            chunk_type,
            chunk_data_bytes: data,
            crc,
        }
    }

    // Returns the length of this Chunk
    fn length(&self) -> u32 {
        *&self.length
    }

    // Returns a reference to this Chunk's ChunkType
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    // Returns the data represented as bytes hidden in this Chunk
    fn data(&self) -> &[u8] {
        &self.chunk_data_bytes[0..*&self.chunk_data_bytes.len()]
    }

    // Returns the crc of this Chunk
    fn crc(&self) -> u32 {
        *&self.crc
    }

    // Returns the data represented as a String hidden in this Chunk
    pub fn data_as_string(&self) -> Result<String, std::io::Error> {
        let data = &self.data();
        let mut data_str = String::new();

        // EFFECT:
        for byte in data.iter() {
            data_str += u8_to_string(*byte).as_str();
        }

        if self.length() != data.len() as u32 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Chunk length doesn't match the data length",
            ));
        }

        Ok(data_str)
    }

    // Returns this Chunk as a list of its bytes. Index 0 - 3 is the length,
    // Index 4 - 7 is the Chunk type. Index 8 - 8 + length is the data
    // and the last 4 indexes are the CRC
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut chunk_as_vec = Vec::<u8>::new();

        chunk_as_vec.extend_from_slice(&u32_to_bytes(self.length));
        chunk_as_vec.extend_from_slice(&self.chunk_type.bytes());

        let mut chunk_data = self.chunk_data_bytes.clone();
        chunk_as_vec.append(&mut chunk_data);
        chunk_as_vec.extend_from_slice(&u32_to_bytes(self.crc));

        chunk_as_vec
    }
}

#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }

    #[test]
    pub fn test_as_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        assert_eq!(chunk_data, chunk.as_bytes());
    }
}
