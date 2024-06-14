// build.rs
fn main() {
    tonic_build::configure()
        .compile(&["../proto/image_transfer.proto"], &["../proto"])
        .unwrap();
}
