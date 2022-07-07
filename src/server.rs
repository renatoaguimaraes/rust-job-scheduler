use env_logger;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

mod lib;
use lib::{Cmd, InMemoryWorker, Worker, WorkerEnum};

pub mod worker {
  tonic::include_proto!("worker");
}

use worker::worker_service_server::{WorkerService, WorkerServiceServer};
use worker::{
  QueryRequest, QueryResponse, StartRequest, StartResponse, StopRequest, StopResponse,
  StreamRequest, StreamResponse,
};

pub struct WorkerServiceServerImpl {
  worker: WorkerEnum,
}

impl Default for WorkerServiceServerImpl {
  fn default() -> Self {
    Self {
      worker: WorkerEnum::InMemoryWorker(InMemoryWorker::default()),
    }
  }
}

#[tonic::async_trait]
impl WorkerService for WorkerServiceServerImpl {
  async fn start(
    &self,
    request: Request<StartRequest>,
  ) -> Result<Response<StartResponse>, tonic::Status> {
    let req = request.into_inner();
    let _ = match self.worker.start(Cmd {
      name: req.name,
      args: req.args,
    }) {
      Ok(uid) => {
        return Ok(Response::new(StartResponse {
          job_id: String::from(uid),
        }))
      }
      Err(msg) => return Err(tonic::Status::internal(msg)),
    };
  }

  async fn stop(
    &self,
    request: Request<StopRequest>,
  ) -> Result<Response<StopResponse>, tonic::Status> {
    let req = request.into_inner();
    match self.worker.stop(req.job_id) {
      Ok(_) => return Ok(Response::new(StopResponse {})),
      Err(err) => return Err(tonic::Status::internal(err.to_string())),
    }
  }

  async fn query(
    &self,
    request: Request<QueryRequest>,
  ) -> Result<Response<QueryResponse>, tonic::Status> {
    let req = request.into_inner();
    println!("query job: {}", req.job_id);
    match self.worker.query(req.job_id) {
      Ok(status) => {
        return Ok(Response::new(QueryResponse {
          pid: status.pid as i32,
          exit_code: status.exit_code,
          exited: status.exited,
        }))
      }
      Err(err) => return Err(tonic::Status::internal(err.to_string())),
    };
  }

  type StreamStream = ReceiverStream<Result<StreamResponse, Status>>;

  async fn stream(
    &self,
    request: Request<StreamRequest>,
  ) -> Result<Response<Self::StreamStream>, tonic::Status> {
    let req = request.into_inner();
    let (tx, rx) = mpsc::channel(4);
    let (mut tx_log, mut rx_log) = mpsc::channel(4);
    let tx = tx.clone();

    let worker = self.worker.clone();
    tokio::spawn(async move {
      worker.stream(req.job_id, &mut tx_log).await;
      while let Some(msg) = rx_log.recv().await {
        let res = StreamResponse {
          output: String::from(format!("{:?}", msg)),
        };
        tx.send(Ok(res)).await.unwrap();
      }
    });
    Ok(Response::new(ReceiverStream::new(rx)))
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
