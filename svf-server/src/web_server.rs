use std::{convert::Infallible, env, net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{serve::IncomingStream, Json, Router};
use axum_server::tls_rustls::RustlsConfig;
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

use crate::{is_production, wait_pool::WaitPool, ServiceHandles};

#[derive(Debug, Serialize, Deserialize)]
pub enum BackendResponse {
    Ok,
    AccessToken(String),
    Error(String),
}

pub fn serve(
    router: Router<Arc<ServiceHandles>>,
    handles: ServiceHandles,
    wait_pool: &mut WaitPool,
) {
    wait_pool.add(tokio::spawn(async {
        if is_production() {
            let config = RustlsConfig::from_pem_chain_file(
                PathBuf::from(env::var("CERT_PATH").expect("No cert var provided")),
                PathBuf::from(env::var("KEY_PATH").expect("No key var provided")),
            )
            .await
            .expect("Invalid certs or key");
            let addr = SocketAddr::new(local_ip().expect("Cannot get local ip"), 443);
            axum_server::bind_rustls(addr, config)
                .serve(router.with_state(Arc::new(handles)).into_make_service())
                .await
                .unwrap();
        } else {
            let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
            axum::serve(listener, router.with_state(Arc::new(handles)))
                .await
                .unwrap();
        }
    }));
}
