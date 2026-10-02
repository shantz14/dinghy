use tonic::{Request, Response, Status};

use std::sync::Mutex;
use std::path::Path;

use hold::kv_service_server::{KvService};
use hold::{GetReply, GetRequest};

use crate::kv::{Store};
use crate::server::hold::{DeleteReply, DeleteRequest, ListReply, ListRequest, PutReply, PutRequest};

const DEFAULT_DATA_PATH: &str = "./data";

pub mod hold {
    tonic::include_proto!("hold"); // The string specified here must match the proto package name
}

#[derive(Debug)]
pub struct MyKvService {
    pub store: Mutex<Store>,
}

struct Flags {
    data_dir: String
}

impl Default for Flags {
    fn default() -> Self {
        Self { data_dir: DEFAULT_DATA_PATH.to_string() }
    }
}

impl MyKvService {
    pub fn new(args: Vec<String>) -> Self {
        let flags = parse_flags(args);
        let path = Path::new(&flags.data_dir);
        Self { store: Mutex::new(Store::new(path).expect("Failed to create Store")) }
    }
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
        self.store.lock().unwrap().put(put_req.key, put_req.value)
            .map_err(|e| Status::internal(format!("PUT write to disk failed: {e}")))?;

        let reply = PutReply{};

        Ok(Response::new(reply))
    }
    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteReply>, Status> {
        println!("Got a request: {:?}", request);

        self.store.lock().unwrap().delete(request.into_inner().key)
            .map_err(|e| Status::internal(format!("DELETE write to disk failed: {e}")))?;

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

fn parse_flags(args: Vec<String>) -> Flags {
    let mut flags = Flags::default();

    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "--data-dir" => {
                if i >= args.len()-1 {
                    panic!("Error parsing flags: no value for --data-dir provided")
                }
                flags.data_dir = args[i+1].clone();
            }
            _ => {}
        }
    }

    flags
}
