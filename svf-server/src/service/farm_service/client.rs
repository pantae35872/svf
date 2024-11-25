use std::net::SocketAddr;

use futures::FutureExt;
use num_enum::TryFromPrimitive;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc::{channel, Sender},
};

use crate::utils::{buffer_reader::BufferReader, buffer_writer::BufferWriter};

use super::{
    packet::{Packet, PacketError, PacketHeader, PacketId},
    server::ServerPacketId,
    ClientPacket, ClientReceiverCommand, ServerPacket,
};

pub struct Client {
    client_sender: Sender<ClientReceiverCommand>,
    stream: TcpStream,
    addr: SocketAddr,
    pending_report: Option<PendingReport>,
    image_buffer: Vec<u8>,
    id: Option<[char; 64]>,
}

struct PendingReport {
    soil_moisture: u16,
    air_temperature: u16,
    light_sensor: u16,
    image_size: usize,
}

#[derive(Debug)]
enum ClientError {
    InvalidPacket(PacketError),
}

impl From<PacketError> for ClientError {
    fn from(value: PacketError) -> Self {
        Self::InvalidPacket(value)
    }
}

impl Client {
    pub fn new(
        client_sender: Sender<ClientReceiverCommand>,
        stream: TcpStream,
        addr: SocketAddr,
    ) -> Self {
        Self {
            client_sender,
            pending_report: None,
            image_buffer: Vec::new(),
            stream,
            id: None,
            addr,
        }
    }

    async fn handle_client_packet(
        &mut self,
        header: PacketHeader,
        sender: Sender<ServerPacket>,
    ) -> Result<(), ClientError> {
        let mut buffer = vec![0u8; header.length() as usize];
        self.stream.read(&mut buffer).await.unwrap();
        let packet = Packet::<ClientPacketId>::new(header.id(), &mut buffer)?;
        match packet.decode()? {
            ClientPacket::ReportId { id } => {
                self.id = Some(id);
                self.client_sender
                    .send(ClientReceiverCommand::ReportClient { id, sender })
                    .await
                    .unwrap();
            }
            ClientPacket::ReportSensors {
                soil_moisture,
                air_temperature,
                light_sensor,
                image_size,
            } => {
                self.pending_report = Some(PendingReport {
                    soil_moisture,
                    air_temperature,
                    light_sensor,
                    image_size,
                });
            }
            ClientPacket::ImageFrame { frame_size, frame } => {
                if let Some(ref report) = self.pending_report {
                    self.image_buffer.extend_from_slice(&frame[0..frame_size]);
                    if self.image_buffer.len() >= report.image_size {
                        self.client_sender
                            .send(ClientReceiverCommand::ReportSensors {
                                id: self.id.expect("Unauthorize"),
                                soil_moisture: report.soil_moisture,
                                air_temperature: report.air_temperature,
                                light_sensor: report.light_sensor,
                                image: self.image_buffer.clone(),
                            })
                            .await
                            .unwrap();
                        self.image_buffer.clear();
                        self.pending_report = None;
                    }
                }
            }
        }
        return Ok(());
    }

    async fn handle_server_packet(&mut self, server_packet: ServerPacket) {
        let mut buffer = Vec::new();
        let mut packet = Packet::<ServerPacketId>::new_packet(&server_packet, &mut buffer).unwrap();
        packet.encode(server_packet);
        self.stream
            .write(&packet.header().to_bytes())
            .await
            .unwrap();
        self.stream.write(&buffer).await.unwrap();
    }

    pub async fn run(&mut self) {
        let mut header_buffer = [0u8; size_of::<PacketHeader>()];
        let (server_sender, mut server_receiver) = channel::<ServerPacket>(16);
        loop {
            let stream_task = self.stream.read(&mut header_buffer).fuse();
            let receiver_task = server_receiver.recv().fuse();
            futures::pin_mut!(stream_task, receiver_task);

            futures::select! {
                readed = stream_task => {
                    let header = PacketHeader::from_bytes(&header_buffer).expect("Should not failed");
                    self.handle_client_packet(header, server_sender.clone()).await;
                    continue;
                }
                packet = receiver_task => {
                    self.handle_server_packet(packet.expect("Packet recv error")).await;
                    continue;
                }
                complete => continue,
            }
        }
    }
}

#[derive(Clone, Copy, TryFromPrimitive)]
#[repr(u32)]
pub enum ClientPacketId {
    ReportId = 0,
    ReportSensors,
    ImageFrame,
}

fn decode_report_id(mut buffer: BufferReader) -> Result<ClientPacket, PacketError> {
    Ok(ClientPacket::ReportId {
        id: buffer
            .const_read_bytes::<64>()
            .map(|e| {
                e.iter()
                    .map(|c| *c as char)
                    .collect::<Vec<char>>()
                    .try_into()
                    .unwrap()
            })
            .ok_or(PacketError::InvalidPacketLength)?,
    })
}

fn decode_sensors(mut buffer: BufferReader) -> Result<ClientPacket, PacketError> {
    let soil_moisture = buffer.read_u16().ok_or(PacketError::InvalidPacketLength)?;
    let air_temperature = buffer.read_u16().ok_or(PacketError::InvalidPacketLength)?;
    let light_sensor = buffer.read_u16().ok_or(PacketError::InvalidPacketLength)?;
    let camera_frames = buffer.read_u64().ok_or(PacketError::InvalidPacketLength)? as usize;

    Ok(ClientPacket::ReportSensors {
        soil_moisture,
        air_temperature,
        light_sensor,
        image_size: camera_frames,
    })
}

fn decode_frame(mut buffer: BufferReader) -> Result<ClientPacket, PacketError> {
    Ok(ClientPacket::ImageFrame {
        frame_size: buffer.read_u64().ok_or(PacketError::InvalidPacketLength)? as usize,
        frame: buffer
            .const_read_bytes::<128>()
            .ok_or(PacketError::InvalidPacketLength)?,
    })
}

impl From<&ClientPacket> for ClientPacketId {
    fn from(value: &ClientPacket) -> Self {
        match value {
            ClientPacket::ReportSensors { .. } => Self::ReportSensors,
            ClientPacket::ImageFrame { .. } => Self::ImageFrame,
            ClientPacket::ReportId { .. } => Self::ReportId,
        }
    }
}

impl PacketId for ClientPacketId {
    type Packet = ClientPacket;

    fn decode(&self, buffer: BufferReader) -> Result<ClientPacket, PacketError> {
        match self {
            Self::ReportId => decode_report_id(buffer),
            Self::ReportSensors => decode_sensors(buffer),
            Self::ImageFrame => decode_frame(buffer),
        }
    }

    fn encode(&self, _buffer: BufferWriter, _packet: ClientPacket) {
        panic!("Operation not supported");
    }

    fn id(&self) -> u32 {
        *self as u32
    }
}
