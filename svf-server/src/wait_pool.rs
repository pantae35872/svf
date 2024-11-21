use tokio::task::JoinHandle;

pub struct WaitPool {
    pool: Vec<JoinHandle<()>>,
}

impl WaitPool {
    pub fn new() -> Self {
        Self { pool: Vec::new() }
    }

    pub fn add(&mut self, handle: JoinHandle<()>) {
        self.pool.push(handle);
    }

    pub async fn wait(self) {
        futures::future::join_all(self.pool).await;
    }
}
