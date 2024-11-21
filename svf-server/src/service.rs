use std::future::Future;

use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    task::JoinHandle,
};

pub mod authentication_service;
pub mod db_service;

pub fn serve_service<T, D, R>(mut service: T) -> JoinHandle<()>
where
    T: Service<D, R> + Send + 'static,
    D: Send,
    R: Send,
{
    tokio::spawn(async move {
        while let Some(request) = service.get_receiver().recv().await {
            let result = service.process(request.data).await;
            request.result_sender.send(result).await.unwrap();
        }
    })
}

pub trait Service<T, R> {
    fn get_sender(&self) -> Sender<ServiceRequest<T, R>>;
    fn get_receiver(&mut self) -> &mut Receiver<ServiceRequest<T, R>>;
    /// Get a handle for a service
    fn get(&self) -> ServiceHandle<T, R> {
        ServiceHandle::<T, R>::new(self.get_sender())
    }

    fn process(&mut self, data: T) -> impl Future<Output = R> + Send;
}

pub struct ServiceRequest<T, R> {
    result_sender: Sender<R>,
    data: T,
}

#[derive(Clone)]
pub struct ServiceHandle<T, R> {
    sender: Sender<ServiceRequest<T, R>>,
}

impl<T, R> ServiceHandle<T, R> {
    fn new(sender: Sender<ServiceRequest<T, R>>) -> Self {
        Self { sender }
    }

    /// Request to a service
    pub async fn request(&self, data: T) -> R {
        let (sender, mut receiver) = channel(1);
        self.sender
            .send(ServiceRequest {
                result_sender: sender,
                data,
            })
            .await
            .expect("A service have been closed already");
        receiver
            .recv()
            .await
            .expect("Failed to receve data from a service")
    }
}
