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
}

impl From<&ServerPacket> for ServerPacketId {
    fn from(value: &ServerPacket) -> Self {
        match value {
            ServerPacket::WaterPulse => Self::WaterPulse,
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
            _ => panic!("Unmatch packet"),
        }
    }

    fn id(&self) -> u32 {
        *self as u32
    }
}
