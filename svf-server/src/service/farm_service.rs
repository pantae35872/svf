use tokio::sync::mpsc::{channel, Receiver, Sender};

pub type ServiceHandle =
    super::ServiceHandle<ServiceRequest, Result<ServiceResponse, ServiceError>>;

type ServiceChannel = super::ServiceRequest<ServiceRequest, Result<ServiceResponse, ServiceError>>;

pub struct Service {
    sender: Sender<ServiceChannel>,
    receiver: Receiver<ServiceChannel>,
}

pub enum ServiceRequest {
    Pair {
        access_token: [char; 128],
        device_id: [char; 64],
    },
}

pub enum ServiceError {}

pub enum ServiceResponse {
    Empty,
}

impl Service {
    pub fn new(db: DBServiceHandle) -> Self {
        let (sender, receiver) = channel(16);
        Self { sender, receiver }
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
