use hex::encode;
use merkle_proof::{MerkleProof, create_proof, load_proof, save_proof, verify};
use sha2::Sha256;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
const LEVELS : usize = 2;
pub fn write_proof<P: AsRef<Path>>(proof: &MerkleProof<LEVELS>, path: P) -> io::Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "[proof]")?;
    writeln!(file, "index = {}", proof.index)?;
    fn write_hash(file: &mut File, hash: [u8; 32]) -> io::Result<()> {
        let hex_string = hex::encode(hash);
        write!(file, "{}", hex_string)?;
        Ok(())
    }

    write!(file, "leaf = ")?;
    write_hash(&mut file, proof.leaf)?;
    writeln!(file)?;
    write!(file, "root = ")?;
    write_hash(&mut file, proof.root)?;
    writeln!(file)?;

    write!(file, "siblings = [")?;
    writeln!(file)?;
    let mut it = proof.siblings.iter().peekable();
    while let Some(sibling) = it.next() {
        write!(file, "    ")?;
        write_hash(&mut file, *sibling)?;
        if it.peek().is_some() {
            write!(file, ",")?;
        }
        writeln!(file)?;
    }
    write!(file, "]")?;
    writeln!(file)?;
    Ok(())
}

fn main() {
    let proof = create_proof::<Sha256, LEVELS>(2);
    println!("Proof created");
    let valid = verify::<Sha256, LEVELS>(proof.clone());
    println!("The proof is valid {}", valid);
    let bytes = save_proof(&proof).expect("serialize");
    println!("bytes len: {}", bytes.len());
    let loaded_proof = load_proof(&bytes).expect("deserialized");
    let valid = verify::<Sha256, LEVELS>(loaded_proof);
    println!("The proof is valid {}", valid);
    write_proof(&proof, "Prover.toml").expect("write to file");
    println!("Proof written to Prover.toml");
}
