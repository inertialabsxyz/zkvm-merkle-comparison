use core::poseidon::hades_permutation;

const LEVELS: u32 = 16;
type Hash = felt252;

pub struct MerkleProof {
    pub root: Hash,
    pub index: u32,
    pub leaf: Hash,
    pub siblings: [Hash; LEVELS]
}

fn hash(left: Hash, right: Hash) -> Hash {
    let (r, _, _) = hades_permutation(left, right, 2);
    r
}

pub fn verify(proof: MerkleProof) {
    let mut current = proof.leaf;
    let mut index = proof.index;

    for sibling in proof
        .siblings
        .span() {
            if index & 1 == 0 {
                current = hash(current, *sibling);
            } else {
                current = hash(*sibling, current);
            }
            index = index / 2;
        };

    assert!(current == proof.root, "invalid proof");
}

#[executable]
fn main(root: Hash, index: u32, leaf: Hash, siblings: [Hash; LEVELS]) {
    verify(MerkleProof { root, leaf, index, siblings });
}
