use starknet_crypto::{Felt, poseidon_hash, poseidon_hash_many};

#[derive(PartialEq, Debug, Clone)]
pub struct MerkleProof<const N: usize> {
    pub root: Felt,
    pub index: usize,
    pub leaf: Felt,
    pub siblings: [Felt; N],
}

pub fn verify<const N: usize>(proof: MerkleProof<N>) -> bool {
    let mut current = proof.leaf;
    let mut index = proof.index;
    for sibling in proof.siblings {
        if index & 1 == 0 {
            current = poseidon_hash(current, sibling);
        } else {
            current = poseidon_hash(sibling, current);
        }
        index >>= 1;
    }

    current == proof.root
}

pub fn create_proof<const N: usize>(index: usize) -> MerkleProof<N> {
    let size = 2u32.pow(N as u32);
    let leaves: Vec<Felt> = (0..size)
        .map(|i: u32| poseidon_hash_many(&vec![i.into()]))
        .collect();
    // Create the bottom layer with the leaves
    let mut layers = vec![leaves];
    // We take the layer in pairs and append the hash to the layer above
    while layers.last().unwrap().len() > 1 {
        let prev = layers.last().unwrap();
        let next: Vec<Felt> = prev
            .chunks(2)
            .map(|pair| poseidon_hash(pair[0], pair[1]))
            .collect();
        layers.push(next);
    }
    let root = layers.last().unwrap()[0];
    let leaf = layers[0][index];
    let mut siblings: Vec<Felt> = Vec::new();
    let mut idx = index;
    for level in 0..N {
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

#[test]
fn test_verify() {
    let index = 2;
    assert!(verify::<2>(create_proof::<2>(index)))
}
