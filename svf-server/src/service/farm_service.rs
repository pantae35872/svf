use std::{collections::HashMap, fs::read_dir, net::SocketAddr, sync::Arc};

use crate::service::db_service::DBServiceResponse;

use super::db_service::{DBServiceHandle, DBServiceRequest};
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

struct ServerClient {
    target_temperature: i32,
    sender: Sender<ServerPacket>,
}

pub struct Service {
    sender: Sender<ServiceChannel>,
    receiver: Receiver<ServiceChannel>,
    db: DBServiceHandle,
    clients: HashMap<[char; 64], ServerClient>,
}

pub enum ServiceRequest {
    Pair {
        access_token: [char; 128],
        device_id: [char; 64],
    },
    ReceiverCommand(ClientReceiverCommand),
}

#[derive(Debug)]
pub enum ServiceError {}

pub enum ServerPacket {
    UpdateCooler { status: bool },
    WaterPulse,
    ResponseId { id: [char; 64] },
}

pub enum ClientReceiverCommand {
    RequestId {
        callback: Sender<[char; 64]>,
        region: String,
    },
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
    RequestId {
        region: String,
    },
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
        mut clients_receiver: Receiver<ClientReceiverCommand>,
    ) {
        while let Some(command) = clients_receiver.recv().await {
            service
                .request(ServiceRequest::ReceiverCommand(command))
                .await
                .unwrap();
        }
    }

    async fn process_sensor(
        &mut self,
        id: [char; 64],
        soil_moisture: u16,
        air_temperature: u16,
        light_sensor: u16,
        image: Vec<u8>,
    ) {
        println!(
            "Id: {}, soil_moisture: {}, air_temperature: {}, light_sensor: {}",
            id.iter().collect::<String>(),
            soil_moisture,
            air_temperature,
            light_sensor
        );
    }

    async fn process_command(&mut self, command: ClientReceiverCommand) {
        match command {
            ClientReceiverCommand::RequestId { callback, region } => {
                let id = match self
                    .db
                    .request(DBServiceRequest::CreateNewDevice { region })
                    .await
                {
                    Ok(DBServiceResponse::DeviceId(id)) => id,
                    Ok(..) => unreachable!(),
                    Err(_) => ['\0'; 64],
                };
                callback.send(id).await.unwrap();
            }
            ClientReceiverCommand::ReportClient { id, sender } => {
                let temperature = match self
                    .db
                    .request(DBServiceRequest::GetTemperature { id })
                    .await
                {
                    Ok(DBServiceResponse::Temperature(temp)) => temp,
                    Ok(..) => unreachable!(),
                    Err(..) => 0,
                };
                self.clients.insert(
                    id,
                    ServerClient {
                        target_temperature: temperature,
                        sender,
                    },
                );
            }
            ClientReceiverCommand::ReportSensors {
                id,
                soil_moisture,
                air_temperature,
                light_sensor,
                image,
            } => {
                self.process_sensor(id, soil_moisture, air_temperature, light_sensor, image)
                    .await
            }
        };
    }

    async fn server_listener(sender: Sender<ClientReceiverCommand>) {
        let listener = TcpListener::bind(SocketAddr::new(
            local_ip().expect("Cannot get local ip"),
            4000,
        ))
        .await
        .expect("Cannot bind to port 4000");
        loop {
            let sender = sender.clone();
            match listener.accept().await {
                Ok((stream, addr)) => tokio::spawn(async move {
                    println!("Incoming connection {addr}");
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
            ServiceRequest::ReceiverCommand(command) => {
                self.process_command(command).await;
                return Ok(ServiceResponse::Empty);
            }
        }
    }
}
