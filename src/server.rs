use env_logger;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

mod lib;

use lib::{Cmd, InMemoryWorker, Worker};

pub mod worker {
    tonic::include_proto!("worker");
}

use worker::worker_service_server::{WorkerService, WorkerServiceServer};
use worker::{
    QueryRequest, QueryResponse, StartRequest, StartResponse, StopRequest, StopResponse,
    StreamRequest, StreamResponse,
};

#[derive(Default)]
pub struct WorkerServiceServerImpl {}

#[tonic::async_trait]
impl WorkerService for WorkerServiceServerImpl {
    async fn start(
        &self,
        request: Request<StartRequest>,
    ) -> Result<Response<StartResponse>, tonic::Status> {
        let req = request.into_inner();
        let mut worker: InMemoryWorker = Worker::new();
        let uid = worker
            .start(Cmd {
                name: req.name,
                args: req.args,
            })
            .unwrap();
        Ok(Response::new(StartResponse { job_id: uid }))
    }

    async fn stop(
        &self,
        request: Request<StopRequest>,
    ) -> Result<Response<StopResponse>, tonic::Status> {
        let req = request.into_inner();
        println!("stop job: {}", req.job_id);
        Ok(Response::new(StopResponse {}))
    }

    async fn query(
        &self,
        request: Request<QueryRequest>,
    ) -> Result<Response<QueryResponse>, tonic::Status> {
        let req = request.into_inner();
        println!("query job: {}", req.job_id);
        Ok(Response::new(QueryResponse {
            pid: 1,
            exit_code: 1,
            exited: false,
        }))
    }

    type StreamStream = ReceiverStream<Result<StreamResponse, Status>>;

    async fn stream(
        &self,
        request: Request<StreamRequest>,
    ) -> Result<Response<Self::StreamStream>, tonic::Status> {
        let req = request.into_inner();
        println!("stream job: {}", req.job_id);
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let addr = "0.0.0.0:50051".parse()?;
    let hello_server = WorkerServiceServerImpl::default();
    Server::builder()
        .add_service(WorkerServiceServer::new(hello_server))
        .serve(addr)
        .await?;
    Ok(())
}
