# Zero-Knowledge Virtual Machine Comparison

A comparative study of different zero-knowledge proof systems implementing identical Merkle tree proof verification across five zkVM frameworks.

## Overview

This project implements the same Merkle tree proof verification algorithm across multiple zero-knowledge virtual machine (zkVM) platforms to enable direct comparison of:

- **Implementation complexity** - Code structure and developer experience
- **Performance characteristics** - Proof generation time and proof size
- **Integration patterns** - How each framework handles guest/host interaction
- **Ecosystem maturity** - Available tooling and documentation

## Merkle Tree Specification

All implementations verify proofs for a **16-level binary Merkle tree** with:
- **65,536 total leaves** (2^16)
- **SHA256 hashing** (except Leo, which uses Poseidon2)
- **16 sibling hashes** per proof
- Binary serialization via bincode (Rust implementations)

The core verification algorithm remains identical across all platforms, ensuring a fair comparison.

## Framework Implementations

### 1. Base Library (`merkle-proof/`)

Pure Rust implementation serving as the reference library.

**Key Components:**
- `MerkleProof` struct with 16-level proof support
- `MerkleHash` trait for pluggable hash functions
- `verify()` and `create_proof()` functions
- Serialization support via serde/bincode

**Dependencies:**
```toml
sha2 = "0.10.9"
bincode = "2.0.1"
serde = { version = "1.0", features = ["derive"] }
```

### 2. SP1 (`sp1/merkle/`)

Succinct Labs' SP1 zkVM implementation.

**Structure:**
- `lib/` - Shared merkle proof library
- `program/` - Guest code (runs inside zkVM)
- `script/` - Host prover code

**Usage:**
```bash
cd sp1/merkle
cargo run --release -- --execute  # Execute without proof
cargo run --release -- --prove    # Generate ZK proof
```

**Features:**
- Supports Groth16 and PLONK proof generation
- EVM-compatible proof export
- SP1 SDK integration

### 3. RISC Zero (`risczero/merkle/`)

RISC Zero zkVM with Bonsai proving service support.

**Structure:**
- `host/` - Host prover code
- `methods/guest/` - Guest code (runs in zkVM)
- `methods/` - Build configuration

**Usage:**
```bash
cd risczero/merkle
cargo run --release
```

**Features:**
- Local proving (dev-mode)
- Remote Bonsai proving service integration
- Execution statistics via RUST_LOG
- Image ID verification

### 4. OpenVM (`openvm/merkle/`)

OpenVM framework implementation.

**Structure:**
- `merkle/` - Main program
- `merkle-lib/` - Shared library
- `encoder/` - Encoder utility
- `openvm.toml` - VM configuration

**Configuration:**
```toml
[app_vm_config]
supported_arch = [
    { rv32i = {} },
    { rv32m = {} },
    { io = {} }
]
```

**Dependencies:**
```toml
openvm = "1.4.1"
```

### 5. Aleo Leo (`leo/merkle/`)

Aleo's Leo domain-specific language implementation.

**Key Differences:**
- Uses **Poseidon2 hash** (field arithmetic optimized)
- Field-based operations instead of RISC-based approach
- Asynchronous transition pattern
- Native Leo syntax

**Leo Code:**
```leo
struct MerkleProof {
    root: field,
    index: u32,
    leaf: field,
    siblings: [field; 16],
}

transition main(public proof: MerkleProof) -> bool {
    return verify_merkle_proof(proof);
}
```

**Usage:**
```bash
cd leo/merkle
leo run main  # Execute program
leo test      # Run tests
```

## Project Structure

```
comparison/
├── merkle-proof/       # Reference Rust library
│   ├── src/lib.rs
│   └── Cargo.toml
├── sp1/
│   └── merkle/
│       ├── lib/        # Shared library
│       ├── program/    # Guest code
│       └── script/     # Host code
├── risczero/
│   └── merkle/
│       ├── host/       # Host prover
│       └── methods/    # Guest code
├── openvm/
│   ├── merkle/         # Main program
│   ├── merkle-lib/     # Library
│   └── encoder/        # Utilities
└── leo/
    ├── merkle/         # Leo program
    └── leo-merkle-proof/  # Rust utilities
```

## Common Patterns

All implementations follow a similar execution flow:

1. **Input**: Serialized Merkle proof (root, leaf, index, siblings)
2. **Guest/Program Execution**: Deserialize and verify proof
3. **Output**: Boolean result or assertion
4. **Proof Generation**: Create zero-knowledge proof of correct execution
5. **Verification**: Verify the ZK proof

## Getting Started

### Prerequisites

**For Rust-based implementations:**
- Rust toolchain (stable or specified version)
- Cargo

**For Leo:**
- Leo CLI (`cargo install leo-lang`)
- Aleo development tools

### Running Comparisons

Each implementation can be run independently. Navigate to the respective directory and use the commands shown above.

## Technology Stack

| Framework | Language | Primary Deps | Hash Function |
|-----------|----------|--------------|---------------|
| merkle-proof | Rust | sha2, bincode | SHA256 |
| SP1 | Rust | sp1-zkvm | SHA256 |
| RISC Zero | Rust | risc0-zkvm | SHA256 |
| OpenVM | Rust | openvm | SHA256 |
| Aleo Leo | Leo | snarkvm | Poseidon2 |

## Benchmarks

_TODO: Add comparative benchmarks for:_
- Proof generation time
- Proof size
- Verification time
- Memory usage
- Lines of code

## Research Questions

This project aims to answer:

1. Which zkVM offers the best developer experience?
2. What are the performance tradeoffs between different proof systems?
3. How do RISC-based VMs compare to field-based systems like Leo?
4. Which framework is best suited for production deployment?
5. What are the integration patterns for each zkVM?

## Contributing

This is a research project. Contributions welcome for:
- Additional zkVM implementations
- Benchmark improvements
- Documentation enhancements
- Algorithm optimizations

## License

_TODO: Add license information_

## References

- [SP1 Documentation](https://docs.succinct.xyz/)
- [RISC Zero Documentation](https://dev.risczero.com/)
- [OpenVM Documentation](https://docs.openvm.dev/)
- [Aleo Leo Documentation](https://developer.aleo.org/)
