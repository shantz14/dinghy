mod server;
mod kv;

use tonic::transport::Server;

use crate::server::{MyKvService, hold::kv_service_server::KvServiceServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let kv_service = MyKvService::default();

    println!("Running hold server on 50051...");

    Server::builder()
        .add_service(KvServiceServer::new(kv_service))
        .serve(addr)
        .await?;

    Ok(())
}
