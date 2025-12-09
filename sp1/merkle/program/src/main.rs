#![no_main]

use merkle_verify::{load_proof, verify};
use sha2::Sha256;
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let proof = sp1_zkvm::io::read_vec();
    let proof = load_proof(&proof).unwrap();
    let proven = verify::<Sha256>(proof);
    sp1_zkvm::io::commit(&proven);
}
