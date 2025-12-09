
use merkle_verify::{create_proof, utils::{load_proof, save_proof, write_proof}, verify};
use snarkvm::prelude::*;

const LEVELS: u32 = 16;

fn main() {
    let proof = create_proof::<TestnetV0, LEVELS>(42);
    println!("Proof created");
    let valid = verify(proof.clone());
    println!("The proof is valid {}", valid);
    let bytes = save_proof(&proof).expect("serialize");
    println!("bytes len: {}", bytes.len());
    let loaded_proof = load_proof::<TestnetV0>(&bytes).expect("deserialized");
    let valid = verify(loaded_proof);
    println!("The proof is valid {}", valid);
    write_proof(&proof, "proof.txt").expect("write to file");
    println!("Proof written to proof.txt");
}
