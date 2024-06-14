// src/main.rs
use tonic::{transport::Channel, Request};
use imagetransfer::image_transfer_client::ImageTransferClient;
use imagetransfer::ImageRequest;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;
use std::path::Path;

pub mod imagetransfer {
    tonic::include_proto!("imagetransfer");
}

async fn send_image(client: &mut ImageTransferClient<Channel>, image_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let image_data = fs::read(image_path)?;
    let image_name = image_path.to_string();

    let start_time = Utc::now();
    let request = Request::new(ImageRequest {
        image_data,
        image_name: image_name.clone(),
    });

    // Record the start time and image name to a CSV file
    let mut file = OpenOptions::new().append(true).create(true).open("client_log.csv")?;
    writeln!(file, "{},{}", image_name, start_time)?;

    let response = client.send_image(request).await?;

    let duration = Utc::now().signed_duration_since(start_time).num_milliseconds();
    println!(
        "Sent image: {}, Latency: {} ms, Response: {:?}",
        image_name,
        duration,
        response.into_inner()
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ImageTransferClient::connect("http://[::1]:50051").await?;

    let image_dir = Path::new("../images");
    let image_paths: Vec<_> = fs::read_dir(image_dir)?
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .map(|e| e.path().to_string_lossy().into_owned())
        .collect();

    for image_path in image_paths {
        if let Err(e) = send_image(&mut client, &image_path).await {
            eprintln!("Failed to send image {}: {}", image_path, e);
        }
    }

    Ok(())
}
