use merkle::{MerkleProof, verify};

#[test]
fn test_with_small_tree() {
    println!("Proving...");
    let proof = MerkleProof {
        root: "b1131d4f6e5ec433ac061dfc821ba4606dfc2920f4e8b58a7c247681a3760de7",
        leaf: "26b25d457597a7b0463f9620f666dd10aa2c4373a505967c7c8d70922a2d6ece",
        index: 2,
        siblings: [
            "9d9f290527a6be626a8f5985b26e19b237b44872b03631811df4416fc1713178",
            "4bda22dd1491025da6af2334021d559e6224cacc07dff8e4e1015671a660c24a"
        ],
    };
    verify(proof);
}
