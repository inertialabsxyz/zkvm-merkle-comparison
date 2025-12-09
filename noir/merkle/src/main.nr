use sha256::digest;

global LEVELS: u32 = 16;
type HASH = [u8; 32];

struct MerkleProof {
    root: HASH,
    index: u32,
    leaf: HASH,
    siblings: [HASH; LEVELS],
}

fn hash(left: HASH, right: HASH) -> HASH {
    let concatenated = left.concat(right);
    digest(concatenated)
}

fn main(proof: MerkleProof) {
    let mut current = proof.leaf;
    let mut index = proof.index;
    for idx in 0u32..LEVELS {
        let sibling = proof.siblings[idx];
        if index & 1 == 0 {
            current = hash(current, sibling);
        } else {
            current = hash(sibling, current);
        }
        index >>= 1u32;
    }
    assert_eq(current, proof.root);
}

#[test]
fn test_main() {
    // main(1, 2);

    // Uncomment to make test fail
    // main(1, 1);
}
