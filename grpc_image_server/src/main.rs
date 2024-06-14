// src/main.rs
use tonic::{transport::Server, Request, Response, Status};
use imagetransfer::image_transfer_server::{ImageTransfer, ImageTransferServer};
use imagetransfer::{ImageRequest, ImageResponse};
use std::time::Instant;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;

pub mod imagetransfer {
    tonic::include_proto!("imagetransfer");
}

#[derive(Debug, Default)]
pub struct MyImageTransfer {}

#[tonic::async_trait]
impl ImageTransfer for MyImageTransfer {
    async fn send_image(
        &self,
        request: Request<ImageRequest>,
    ) -> Result<Response<ImageResponse>, Status> {
        let start_time = Instant::now();
        let image = request.into_inner();

        let receive_time = Utc::now();
        println!("Received image: {}", image.image_name);

        // Record the receive time and image name to a CSV file
        let mut file = OpenOptions::new().append(true).create(true).open("server_log.csv")?;
        writeln!(file, "{},{}", image.image_name, receive_time)?;

        let duration = start_time.elapsed();
        let response = ImageResponse {
            status: "Received".into(),
            latency_ms: duration.as_millis() as i64,
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let image_transfer = MyImageTransfer::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(ImageTransferServer::new(image_transfer))
        .serve(addr)
        .await?;

    Ok(())
}
