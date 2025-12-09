use merkle_lib::{create_proof, save_proof};
use sha2::Sha256;
use std::error::Error;
use std::{fs, path::PathBuf};

fn encode_input_file<T>(value: &T, path: PathBuf) -> Result<(), Box<dyn Error>>
where
    T: serde::Serialize + ?Sized,
{
    let words = openvm::serde::to_vec(value)?;
    let bytes: Vec<u8> = words.into_iter().flat_map(|w| w.to_le_bytes()).collect();
    let hex_bytes = String::from("0x01") + &hex::encode(&bytes);
    let input = serde_json::json!({
        "input": [hex_bytes]
    });
    fs::write(path, serde_json::to_string(&input)?)?;
    Ok(())
}

fn main() {
    let proof = create_proof::<Sha256>(42);
    let out = save_proof(&proof).expect("serialized proof");
    println!("Proof length: {}", out.len());
    let path = PathBuf::from("/tmp/proof.bin");
    encode_input_file(&out, path).expect("Proof failed");
    println!("Proof written");
}
