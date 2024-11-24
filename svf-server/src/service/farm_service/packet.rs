use std::{error::Error, fmt::Display};

use crate::utils::{buffer_reader::BufferReader, buffer_writer::BufferWriter};

pub struct Packet<'a, T: PacketId + TryFrom<u32>> {
    id: T,
    buffer: &'a mut Vec<u8>,
}

pub struct PacketHeader {
    length: u32,
    id: u32,
}

impl PacketHeader {
    pub fn length(&self) -> u32 {
        return self.length;
    }

    pub fn id(&self) -> u32 {
        return self.id;
    }

    pub fn from_bytes(buffer: &[u8]) -> Result<Self, PacketError> {
        let mut reader = BufferReader::new(buffer);
        Ok(Self {
            length: reader.read_u32().ok_or(PacketError::InvalidPacketLength)?,
            id: reader.read_u32().ok_or(PacketError::InvalidPacketLength)?,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut buffer_writer = BufferWriter::new(&mut buffer);
        buffer_writer.write_u32(self.length);
        buffer_writer.write_u32(self.id);
        buffer
    }
}

impl<'a, T: PacketId + TryFrom<u32> + for<'b> TryFrom<&'b T::Packet>> Packet<'a, T> {
    pub fn new(id: u32, buffer: &'a mut Vec<u8>) -> Result<Self, PacketError> {
        Ok(Self {
            id: T::try_from(id).map_err(|_| PacketError::InvalidPacketId)?,
            buffer,
        })
    }

    pub fn new_packet<'b>(
        packet: &'b T::Packet,
        buffer: &'a mut Vec<u8>,
    ) -> Result<Self, PacketError> {
        Ok(Self {
            id: T::try_from(packet).map_err(|_| PacketError::InvalidPacketId)?,
            buffer,
        })
    }

    pub fn encode(&mut self, packet: T::Packet) -> &mut Self {
        self.id.encode(BufferWriter::new(&mut self.buffer), packet);
        self
    }

    pub fn decode(&self) -> Result<T::Packet, PacketError> {
        self.id.decode(BufferReader::new(self.buffer))
    }

    pub fn header(&self) -> PacketHeader {
        PacketHeader {
            length: self.buffer.len() as u32,
            id: self.id.id(),
        }
    }
}

#[derive(Debug)]
pub enum PacketError {
    InvalidPacketLength,
    InvalidPacketId,
}

impl Display for PacketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPacketLength => write!(f, "Invalid packet length"),
            Self::InvalidPacketId => write!(f, "Invalid packet id"),
        }
    }
}

impl Error for PacketError {}

pub trait PacketId {
    type Packet;

    fn decode(&self, buffer: BufferReader) -> Result<Self::Packet, PacketError>;

    fn encode(&self, buffer: BufferWriter, packet: Self::Packet);

    fn id(&self) -> u32;
}
