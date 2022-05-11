use tonic::Request;

pub mod worker {
    tonic::include_proto!("worker");
}

use worker::worker_service_client::WorkerServiceClient;
use worker::StartRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // creating a channel ie connection to server
    let channel = tonic::transport::Channel::from_static("http://0.0.0.0:50051")
        .connect()
        .await?;
    // creating gRPC client from channel
    let mut client = WorkerServiceClient::new(channel);
    // creating a new Request
    let request = Request::new(StartRequest {
        name: String::from("echo"),
        args: vec![String::from("1"), String::from("2")],
    });
    // sending request and waiting for response
    let response = client.start(request).await?.into_inner();
    println!("response={:?}", response);
    Ok(())
}
