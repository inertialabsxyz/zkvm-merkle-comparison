// The 'merkle' program.
program merkle_verify.aleo {
    // This is the constructor for the program.
    // The constructor allows you to manage program upgrades.
    // It is called when the program is deployed or upgraded.
    // It is currently configured to **prevent** upgrades.
    // Other configurations include:
    //  - @admin(address="aleo1...")
    //  - @checksum(mapping="credits.aleo/fixme", key="0field")
    //  - @custom
    // For more information, please refer to the documentation: `https://docs.leo-lang.org/guides/upgradability`
    @noupgrade
    async constructor() {}

    const LEVELS: u32 = 16;
    struct MerkleProof {
        root: field,
        index: u32,
        leaf: field,
        siblings: [field; LEVELS],
    }

    transition test_hash(left: field, right: field) -> field {
        return hash(left, right);
    }

    function hash(left: field, right:field) -> field {
        let result = Poseidon2::hash_to_field_raw([left, right]);
        return result;
    }

    transition main(public proof: MerkleProof) -> bool {
        let current = proof.leaf;
        let index = proof.index;
        for idx in 0u32..LEVELS {
            let sibling = proof.siblings[idx];
            if index & 1 == 0 {
                current = hash(current, sibling);
            } else {
                current = hash(sibling, current);
            }
            index >>= 1u32;
        }
        return current == proof.root;
    }
}
