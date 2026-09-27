use tonic::{Request, Response, Status};

use std::sync::Mutex;

use hold::kv_service_server::{KvService};
use hold::{GetReply, GetRequest};

use crate::kv::{Store};
use crate::server::hold::{DeleteReply, DeleteRequest, ListReply, ListRequest, PutReply, PutRequest};

pub mod hold {
    tonic::include_proto!("hold"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyKvService {
    store: Mutex<Store>
}

#[tonic::async_trait]
impl KvService for MyKvService {
    async fn get(
        &self,
        request: Request<GetRequest>,
    ) -> Result<Response<GetReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = GetReply {
            // TODO implement
            kv: self.store.lock().unwrap().get(request.into_inner().key)
        };

        Ok(Response::new(reply))
    }
    async fn put(
        &self,
        request: Request<PutRequest>,
    ) -> Result<Response<PutReply>, Status> {
        println!("Got a request: {:?}", request);

        let put_req = request.into_inner();
        self.store.lock().unwrap().put(put_req.key, put_req.value);

        let reply = PutReply{};

        Ok(Response::new(reply))
    }
    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteReply>, Status> {
        println!("Got a request: {:?}", request);

        self.store.lock().unwrap().delete(request.into_inner().key);

        let reply = DeleteReply{};

        Ok(Response::new(reply))
    }
    async fn list(
        &self,
        request: Request<ListRequest>,
    ) -> Result<Response<ListReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = ListReply {
            //TODO implement
            list: self.store.lock().unwrap().range(request.into_inner().prefix),
        };

        Ok(Response::new(reply))
    }
}

