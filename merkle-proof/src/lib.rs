type Bytes32 = [u8; 32];
const LEVELS: usize = 16;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};
use sha2::{Digest, Sha256};
pub trait MerkleHash {
    fn hash<T: AsRef<[u8]>>(_: T) -> Bytes32 {
        [0; 32]
    }
}

impl MerkleHash for () {}

impl MerkleHash for Sha256 {
    fn hash<T: AsRef<[u8]>>(input: T) -> Bytes32 {
        Sha256::digest(input).into()
    }
}

fn append32(a: &[u8; 32], b: &[u8; 32]) -> [u8; 64] {
    let mut out = [0u8; 64];
    out[..32].copy_from_slice(a);
    out[32..].copy_from_slice(b);
    out
}

pub fn verify<H: MerkleHash>(proof: MerkleProof) -> bool {
    let mut current = proof.leaf;
    let mut index = proof.index;
    for sibling in proof.siblings {
        if index & 1 == 0 {
            current = H::hash(&append32(&current, &sibling));
        } else {
            current = H::hash(&append32(&sibling, &current));
        }
        index >>= 1;
    }

    current == proof.root
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct MerkleProof {
    root: Bytes32,
    index: usize,
    leaf: Bytes32,
    siblings: [Bytes32; LEVELS],
}

pub fn create_proof<H: MerkleHash>(index: usize) -> MerkleProof {
    let leaves: Vec<Bytes32> = (0..65536).map(|i: i32| H::hash(&i.to_le_bytes())).collect();
    // Create the bottom layer with the leaves
    let mut layers = vec![leaves];
    // We take the layer in pairs and append the hash to the layer above
    while layers.last().unwrap().len() > 1 {
        let prev = layers.last().unwrap();
        let next: Vec<[u8; 32]> = prev
            .chunks(2)
            .map(|pair| H::hash(&[pair[0], pair[1]].concat()))
            .collect();
        layers.push(next);
    }
    let root = layers.last().unwrap()[0];
    let leaf = layers[0][index];
    let mut siblings: Vec<[u8; 32]> = Vec::new();
    let mut idx = index;
    for level in 0..16 {
        let sibling_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
        siblings.push(layers[level][sibling_idx]);
        idx /= 2;
    }

    MerkleProof {
        root,
        index,
        leaf,
        siblings: siblings.try_into().unwrap(),
    }
}

pub fn save_proof(proof: &MerkleProof) -> Result<Vec<u8>, EncodeError> {
    bincode::encode_to_vec(proof, bincode::config::standard())
}

pub fn load_proof(bytes: &[u8]) -> Result<MerkleProof, DecodeError> {
    let (proof, _) = bincode::decode_from_slice(bytes, bincode::config::standard())?;
    Ok(proof)
}

#[test]
fn test_verify() {
    let index = 2345;
    assert!(verify::<Sha256>(create_proof::<Sha256>(index)))
}

#[test]
fn save_and_load() {
    let proof = create_proof::<Sha256>(42);
    let out = save_proof(&proof).expect("serialized proof");
    let proof_out = load_proof(&out).expect("deserialized proof");
    assert_eq!(proof, proof_out);
}
