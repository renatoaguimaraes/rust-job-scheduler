#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartRequest {
  #[prost(string, tag = "1")]
  pub name: ::prost::alloc::string::String,
  #[prost(string, repeated, tag = "2")]
  pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartResponse {
  #[prost(string, tag = "1")]
  pub job_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {
  #[prost(string, tag = "1")]
  pub job_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
  #[prost(string, tag = "1")]
  pub job_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponse {
  #[prost(int32, tag = "1")]
  pub pid: i32,
  #[prost(int32, tag = "2")]
  pub exit_code: i32,
  #[prost(bool, tag = "3")]
  pub exited: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamRequest {
  #[prost(string, tag = "1")]
  pub job_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamResponse {
  #[prost(string, tag = "1")]
  pub output: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod worker_service_client {
  #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
  use tonic::codegen::*;
  #[derive(Debug, Clone)]
  pub struct WorkerServiceClient<T> {
    inner: tonic::client::Grpc<T>,
  }
  impl WorkerServiceClient<tonic::transport::Channel> {
    #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
    pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
      D: std::convert::TryInto<tonic::transport::Endpoint>,
      D::Error: Into<StdError>,
    {
      let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
      Ok(Self::new(conn))
    }
  }
  impl<T> WorkerServiceClient<T>
  where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::ResponseBody: Body + Send + Sync + 'static,
    T::Error: Into<StdError>,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
  {
    pub fn new(inner: T) -> Self {
      let inner = tonic::client::Grpc::new(inner);
      Self { inner }
    }
    pub fn with_interceptor<F>(
      inner: T,
      interceptor: F,
    ) -> WorkerServiceClient<InterceptedService<T, F>>
    where
      F: tonic::service::Interceptor,
      T: tonic::codegen::Service<
        http::Request<tonic::body::BoxBody>,
        Response = http::Response<
          <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
        >,
      >,
      <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
        Into<StdError> + Send + Sync,
    {
      WorkerServiceClient::new(InterceptedService::new(inner, interceptor))
    }
    #[doc = r" Compress requests with `gzip`."]
    #[doc = r""]
    #[doc = r" This requires the server to support it otherwise it might respond with an"]
    #[doc = r" error."]
    pub fn send_gzip(mut self) -> Self {
      self.inner = self.inner.send_gzip();
      self
    }
    #[doc = r" Enable decompressing responses with `gzip`."]
    pub fn accept_gzip(mut self) -> Self {
      self.inner = self.inner.accept_gzip();
      self
    }
    pub async fn start(
      &mut self,
      request: impl tonic::IntoRequest<super::StartRequest>,
    ) -> Result<tonic::Response<super::StartResponse>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/Worker.WorkerService/Start");
      self.inner.unary(request.into_request(), path, codec).await
    }
    pub async fn stop(
      &mut self,
      request: impl tonic::IntoRequest<super::StopRequest>,
    ) -> Result<tonic::Response<super::StopResponse>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/Worker.WorkerService/Stop");
      self.inner.unary(request.into_request(), path, codec).await
    }
    pub async fn query(
      &mut self,
      request: impl tonic::IntoRequest<super::QueryRequest>,
    ) -> Result<tonic::Response<super::QueryResponse>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/Worker.WorkerService/Query");
      self.inner.unary(request.into_request(), path, codec).await
    }
    pub async fn stream(
      &mut self,
      request: impl tonic::IntoRequest<super::StreamRequest>,
    ) -> Result<tonic::Response<tonic::codec::Streaming<super::StreamResponse>>, tonic::Status>
    {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/Worker.WorkerService/Stream");
      self
        .inner
        .server_streaming(request.into_request(), path, codec)
        .await
    }
  }
}
#[doc = r" Generated server implementations."]
pub mod worker_service_server {
  #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
  use tonic::codegen::*;
  #[doc = "Generated trait containing gRPC methods that should be implemented for use with WorkerServiceServer."]
  #[async_trait]
  pub trait WorkerService: Send + Sync + 'static {
    async fn start(
      &self,
      request: tonic::Request<super::StartRequest>,
    ) -> Result<tonic::Response<super::StartResponse>, tonic::Status>;
    async fn stop(
      &self,
      request: tonic::Request<super::StopRequest>,
    ) -> Result<tonic::Response<super::StopResponse>, tonic::Status>;
    async fn query(
      &self,
      request: tonic::Request<super::QueryRequest>,
    ) -> Result<tonic::Response<super::QueryResponse>, tonic::Status>;
    #[doc = "Server streaming response type for the Stream method."]
    type StreamStream: futures_core::Stream<Item = Result<super::StreamResponse, tonic::Status>>
      + Send
      + Sync
      + 'static;
    async fn stream(
      &self,
      request: tonic::Request<super::StreamRequest>,
    ) -> Result<tonic::Response<Self::StreamStream>, tonic::Status>;
  }
  #[derive(Debug)]
  pub struct WorkerServiceServer<T: WorkerService> {
    inner: _Inner<T>,
    accept_compression_encodings: (),
    send_compression_encodings: (),
  }
  struct _Inner<T>(Arc<T>);
  impl<T: WorkerService> WorkerServiceServer<T> {
    pub fn new(inner: T) -> Self {
      let inner = Arc::new(inner);
      let inner = _Inner(inner);
      Self {
        inner,
        accept_compression_encodings: Default::default(),
        send_compression_encodings: Default::default(),
      }
    }
    pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
    where
      F: tonic::service::Interceptor,
    {
      InterceptedService::new(Self::new(inner), interceptor)
    }
  }
  impl<T, B> tonic::codegen::Service<http::Request<B>> for WorkerServiceServer<T>
  where
    T: WorkerService,
    B: Body + Send + Sync + 'static,
    B::Error: Into<StdError> + Send + 'static,
  {
    type Response = http::Response<tonic::body::BoxBody>;
    type Error = Never;
    type Future = BoxFuture<Self::Response, Self::Error>;
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
      Poll::Ready(Ok(()))
    }
    fn call(&mut self, req: http::Request<B>) -> Self::Future {
      let inner = self.inner.clone();
      match req.uri().path() {
        "/Worker.WorkerService/Start" => {
          #[allow(non_camel_case_types)]
          struct StartSvc<T: WorkerService>(pub Arc<T>);
          impl<T: WorkerService> tonic::server::UnaryService<super::StartRequest> for StartSvc<T> {
            type Response = super::StartResponse;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::StartRequest>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).start(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = StartSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/Worker.WorkerService/Stop" => {
          #[allow(non_camel_case_types)]
          struct StopSvc<T: WorkerService>(pub Arc<T>);
          impl<T: WorkerService> tonic::server::UnaryService<super::StopRequest> for StopSvc<T> {
            type Response = super::StopResponse;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::StopRequest>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).stop(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = StopSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/Worker.WorkerService/Query" => {
          #[allow(non_camel_case_types)]
          struct QuerySvc<T: WorkerService>(pub Arc<T>);
          impl<T: WorkerService> tonic::server::UnaryService<super::QueryRequest> for QuerySvc<T> {
            type Response = super::QueryResponse;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::QueryRequest>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).query(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = QuerySvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/Worker.WorkerService/Stream" => {
          #[allow(non_camel_case_types)]
          struct StreamSvc<T: WorkerService>(pub Arc<T>);
          impl<T: WorkerService> tonic::server::ServerStreamingService<super::StreamRequest>
            for StreamSvc<T>
          {
            type Response = super::StreamResponse;
            type ResponseStream = T::StreamStream;
            type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<super::StreamRequest>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).stream(request).await };
              Box::pin(fut)
            }
          }
          let accept_compression_encodings = self.accept_compression_encodings;
          let send_compression_encodings = self.send_compression_encodings;
          let inner = self.inner.clone();
          let fut = async move {
            let inner = inner.0;
            let method = StreamSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = tonic::server::Grpc::new(codec)
              .apply_compression_config(accept_compression_encodings, send_compression_encodings);
            let res = grpc.server_streaming(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        _ => Box::pin(async move {
          Ok(
            http::Response::builder()
              .status(200)
              .header("grpc-status", "12")
              .header("content-type", "application/grpc")
              .body(empty_body())
              .unwrap(),
          )
        }),
      }
    }
  }
  impl<T: WorkerService> Clone for WorkerServiceServer<T> {
    fn clone(&self) -> Self {
      let inner = self.inner.clone();
      Self {
        inner,
        accept_compression_encodings: self.accept_compression_encodings,
        send_compression_encodings: self.send_compression_encodings,
      }
    }
  }
  impl<T: WorkerService> Clone for _Inner<T> {
    fn clone(&self) -> Self {
      Self(self.0.clone())
    }
  }
  impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self.0)
    }
  }
  impl<T: WorkerService> tonic::transport::NamedService for WorkerServiceServer<T> {
    const NAME: &'static str = "Worker.WorkerService";
  }
}
