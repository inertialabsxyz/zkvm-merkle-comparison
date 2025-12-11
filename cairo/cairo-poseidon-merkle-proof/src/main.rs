use cairo_poseidon_merkle_proof::{create_proof, verify};
const LEVELS: usize = 16;

fn main() {
    let proof = create_proof::<LEVELS>(42);
    println!("Proof created");
    let valid = verify(proof.clone());
    println!("The proof is valid {}", valid);

    // We want a list of items to pass on the command line.
    // fn main(root: Hash, index: u32, leaf: Hash, siblings: [Hash; LEVELS])
    print!("{},{},{}", proof.root, proof.index, proof.leaf);
    for sibling in proof.siblings.iter() {
        print!(",{}", sibling);
    }
}
