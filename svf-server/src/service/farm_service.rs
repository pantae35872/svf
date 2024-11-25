use std::{collections::HashMap, fs::read_dir, net::SocketAddr, sync::Arc};

use crate::service::db_service::DBServiceResponse;

use super::db_service::{DBServiceHandle, DBServiceRequest};
use client::{Client, ClientPacketId};
use futures::FutureExt;
use local_ip_address::local_ip;
use packet::{Packet, PacketHeader};
use sha2::digest::typenum::TArr;
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
    test_image: Option<Vec<u8>>,
}

pub enum ServiceRequest {
    Pair {
        access_token: [char; 128],
        device_id: [char; 64],
    },
    Image,
    ReceiverCommand(ClientReceiverCommand),
}

#[derive(Debug)]
pub enum ServiceError {}

pub enum ServerPacket {
    UpdateCooler { status: bool },
    WaterPulse,
}

pub enum ClientReceiverCommand {
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
    Image(Option<Vec<u8>>),
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
            test_image: None,
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
        loop {
            let command = match clients_receiver.recv().await {
                Some(command) => command,
                None => continue,
            };
            match service
                .request(ServiceRequest::ReceiverCommand(command))
                .await
            {
                Ok(_) => {}
                Err(err) => {
                    println!("{err:?}");
                }
            };
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
        let client = match self.clients.get(&id) {
            Some(client) => client,
            None => {
                println!("Unknown client with id {}", id.iter().collect::<String>());
                return;
            }
        };
        client
            .sender
            .send(ServerPacket::UpdateCooler {
                status: (air_temperature as i32) < client.target_temperature,
            })
            .await
            .unwrap();
        if soil_moisture > 500 {
            client.sender.send(ServerPacket::WaterPulse).await.unwrap();
        }

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
            ServiceRequest::Image => self.get_image().await,
            ServiceRequest::ReceiverCommand(command) => {
                self.process_command(command).await;
                return Ok(ServiceResponse::Empty);
            }
        }
    }
}
