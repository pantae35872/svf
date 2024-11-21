use dotenv::dotenv;
use svf_server::{wait_pool::WaitPool, web_server};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut wait_pool = WaitPool::new();
    let services = svf_server::init_services(&mut wait_pool).await;
    web_server::serve(svf_server::router(), services, &mut wait_pool);
    wait_pool.wait().await;
}
