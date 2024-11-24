use num_enum::TryFromPrimitive;

use crate::utils::{buffer_reader::BufferReader, buffer_writer::BufferWriter};

use super::{
    packet::{PacketError, PacketId},
    ServerPacket,
};

#[derive(Clone, Copy, TryFromPrimitive)]
#[repr(u32)]
pub enum ServerPacketId {
    UpdateCooler = 0,
    WaterPulse,
    ResponseId,
}

impl From<&ServerPacket> for ServerPacketId {
    fn from(value: &ServerPacket) -> Self {
        match value {
            ServerPacket::WaterPulse => Self::WaterPulse,
            ServerPacket::ResponseId { .. } => Self::ResponseId,
            ServerPacket::UpdateCooler { .. } => Self::UpdateCooler,
        }
    }
}

impl PacketId for ServerPacketId {
    type Packet = ServerPacket;

    fn decode(&self, _buffer: BufferReader) -> Result<ServerPacket, PacketError> {
        panic!("Operation not supported on the server")
    }

    fn encode(&self, mut buffer: BufferWriter, packet: ServerPacket) {
        match (self, packet) {
            (Self::UpdateCooler, ServerPacket::UpdateCooler { status }) => {
                buffer.write_bool(status);
            }
            (Self::WaterPulse, ServerPacket::WaterPulse) => {}
            (Self::ResponseId, ServerPacket::ResponseId { id }) => {
                id.iter().for_each(|e| {
                    buffer.write_u8(*e as u8);
                });
            }
            _ => panic!("Unmatch packet"),
        }
    }

    fn id(&self) -> u32 {
        *self as u32
    }
}
