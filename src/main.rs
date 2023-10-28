use rust_kzg::kzg::{blob_to_kzg_commitment, Blob, KzgSettings, BYTES_PER_BLOB};
use std::path::PathBuf;

fn main() {
    // TODO: Convert json file to .txt
    let trusted_setup_path =
        PathBuf::from("/home/echo/Desktop/Ethereum/my-projects/rust_kzg/src/trusted_setup.json");

    let blob = [0u8; BYTES_PER_BLOB];
    let blob = Blob::from_bytes(&blob).expect("hard coded arg works");

    let kzg = KzgSettings::load_trusted_setup_file(trusted_setup_path)
        .expect("can load KZG settings from file");

    println!("Trusted Setup: {:?}", kzg);
    blob_to_kzg_commitment(&blob, &kzg);
}
