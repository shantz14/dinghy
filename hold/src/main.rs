mod server;
mod kv;
mod wal;
mod snapshot;

use std::{env};

use tonic::transport::Server;

use crate::{server::{MyKvService, hold::kv_service_server::KvServiceServer}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let addr = "[::1]:50051".parse()?;
    let kv_service = MyKvService::new(args);

    println!("Running hold server on 50051...");

    Server::builder()
        .add_service(KvServiceServer::new(kv_service))
        .serve(addr)
        .await?;

    Ok(())
}
