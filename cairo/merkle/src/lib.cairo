use core::sha256::compute_sha256_byte_array;

const LEVELS: u32 = 2;
type HASH = ByteArray;

pub struct MerkleProof {
    pub root: HASH,
    pub index: u32,
    pub leaf: HASH,
    pub siblings: [HASH; LEVELS]
}

fn hex_char_to_nibble(c: u8) -> u8 {
    if c >= '0' && c <= '9' {
        c - '0'
    } else if c >= 'a' && c <= 'f' {
        c - 'a' + 10
    } else {
        c - 'A' + 10
    }
}

fn hex_to_bytes(hex: @ByteArray) -> ByteArray {
    let mut result: ByteArray = "";
    let mut i: usize = 0;
    
    while i < hex.len() {
        let high = hex_char_to_nibble(hex[i]);
        let low = hex_char_to_nibble(hex[i + 1]);
        result.append_byte(high * 16 + low);
        i += 2;
    };
    
    result
}

fn u32_array_to_bytes(hash: [u32; 8]) -> ByteArray {
    let mut result: ByteArray = "";

    for word in hash.span() {
        result.append_word((*word).into(), 4);
    };

    result
}

fn hash(left: @HASH, right: @HASH) -> HASH {
    let to_hash = left.clone() + right.clone();
    let result = compute_sha256_byte_array(@to_hash);
    u32_array_to_bytes(result)
}

pub fn verify(proof: MerkleProof) {
    // Convert all hex inputs to bytes
    let root = hex_to_bytes(@proof.root);
    let mut current = hex_to_bytes(@proof.leaf);
    let mut index = proof.index;
    
    for sibling in proof.siblings.span() {
        let sibling_bytes = hex_to_bytes(sibling);
        if index & 1 == 0 {
            current = hash(@current, @sibling_bytes);
        } else {
            current = hash(@sibling_bytes, @current);
        }
        index = index / 2;
    };

    assert!(current == root, "invalid proof");
}

// #[executable]
// fn main() {
//     println!("Proving...");
//     let proof = MerkleProof {
//         root: "b1131d4f6e5ec433ac061dfc821ba4606dfc2920f4e8b58a7c247681a3760de7",
//         leaf: "26b25d457597a7b0463f9620f666dd10aa2c4373a505967c7c8d70922a2d6ece",
//         index: 2,
//         siblings: [
//             "9d9f290527a6be626a8f5985b26e19b237b44872b03631811df4416fc1713178",
//             "4bda22dd1491025da6af2334021d559e6224cacc07dff8e4e1015671a660c24a"
//         ],
//     };
//     verify(proof);
// }