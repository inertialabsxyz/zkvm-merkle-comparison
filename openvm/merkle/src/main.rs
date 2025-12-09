use merkle_lib::{load_proof, verify};
use sha2::Sha256;

fn main() {
    let proof = openvm::io::read::<Vec<u8>>();
    let proof = load_proof(&proof).unwrap();
    let valid = verify::<Sha256>(proof);
    assert!(valid);
}
