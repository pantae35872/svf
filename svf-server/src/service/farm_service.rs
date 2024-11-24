use std::{collections::HashMap, fs::read_dir, net::SocketAddr, sync::Arc};

use super::db_service::DBServiceHandle;
use client::{Client, ClientPacketId};
use futures::FutureExt;
use local_ip_address::local_ip;
use packet::{Packet, PacketHeader};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
    select,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
};

mod client;
mod packet;
mod server;

pub type ServiceHandle =
    super::ServiceHandle<ServiceRequest, Result<ServiceResponse, ServiceError>>;

type ServiceChannel = super::ServiceRequest<ServiceRequest, Result<ServiceResponse, ServiceError>>;

pub struct Service {
    sender: Sender<ServiceChannel>,
    receiver: Receiver<ServiceChannel>,
    db: DBServiceHandle,
    clients: HashMap<[char; 64], Sender<ServerPacket>>,
}

pub enum ServiceRequest {
    Pair {
        access_token: [char; 128],
        device_id: [char; 64],
    },
}

pub enum ServiceError {}

pub enum ServerPacket {
    UpdateCooler { status: bool },
    WaterPulse,
    ResponseId { id: [char; 64] },
}

enum ClientReceiverCommand {
    RequestId(Sender<[char; 64]>),
    ReportClient {
        id: [char; 64],
        sender: Sender<ServerPacket>,
    },
    ReportSensors {
        id: [char; 64],
        soil_moisture: u16,
        air_temperature: u16,
        light_sensor: u16,
        image: Vec<u8>,
    },
}

pub enum ClientPacket {
    ReportId {
        id: [char; 64],
    },
    RequestId,
    ReportSensors {
        soil_moisture: u16,
        air_temperature: u16,
        light_sensor: u16,
        image_size: usize,
    },
    ImageFrame {
        frame_size: usize,
        frame: [u8; 128],
    },
}

pub enum ServiceResponse {
    Empty,
}

impl Service {
    pub fn new(db: DBServiceHandle) -> Self {
        let (sender, receiver) = channel(16);
        let (client_sender, clients_receiver) = channel(64);
        tokio::spawn(Self::server_listener(client_sender));
        let service = Self {
            sender,
            receiver,
            clients: HashMap::new(),
            db,
        };
        tokio::spawn(Self::server_main(
            super::Service::get(&service),
            clients_receiver,
        ));
        service
    }

    async fn server_main(
        service: ServiceHandle,
        clients_receiver: Receiver<ClientReceiverCommand>,
    ) {
    }

    async fn server_listener(sender: Sender<ClientReceiverCommand>) {
        let listener = TcpListener::bind(SocketAddr::new(
            local_ip().expect("Cannot get local ip"),
            3000,
        ))
        .await
        .expect("Cannot bind to port 3000");
        loop {
            let sender = sender.clone();
            match listener.accept().await {
                Ok((stream, addr)) => tokio::spawn(async move {
                    let mut client = Client::new(sender, stream, addr);
                    client.run().await;
                }),
                Err(error) => {
                    println!("Failed to connect with a device {error}");
                    continue;
                }
            };
        }
    }
}

impl super::Service<ServiceRequest, Result<ServiceResponse, ServiceError>> for Service {
    fn get_sender(
        &self,
    ) -> Sender<super::ServiceRequest<ServiceRequest, Result<ServiceResponse, ServiceError>>> {
        self.sender.clone()
    }

    fn get_receiver(
        &mut self,
    ) -> &mut Receiver<super::ServiceRequest<ServiceRequest, Result<ServiceResponse, ServiceError>>>
    {
        &mut self.receiver
    }

    async fn process(&mut self, data: ServiceRequest) -> Result<ServiceResponse, ServiceError> {
        match data {
            ServiceRequest::Pair {
                access_token,
                device_id,
            } => Ok(ServiceResponse::Empty),
        }
    }
}
