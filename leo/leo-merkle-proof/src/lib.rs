use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};
use rayon::prelude::*;
use snarkvm::prelude::*;

pub fn hash<T: Environment + Network>(fields: Vec<Field<T>>) -> Field<T> {
    let mut bits = Vec::new();
    for field in fields {
        bits.extend(field.to_bits_le());
    }

    let packed: Vec<Field<T>> = bits
        .chunks(Field::<T>::size_in_data_bits())
        .map(|chunk| Field::from_bits_le(chunk).unwrap())
        .collect();

    T::hash_psd2(&packed).unwrap()
}

pub fn verify<T: Environment + Network>(proof: MerkleProof<T>) -> bool {
    let mut current = proof.leaf.into();
    let mut index = proof.index;
    for sibling in proof.siblings.into_iter().map(Field::from) {
        if index & 1 == 0 {
            current = hash(vec![current, sibling]);
        } else {
            current = hash(vec![sibling, current]);
        }
        index >>= 1;
    }

    current == proof.root.into()
}

#[derive(PartialEq, Debug, Clone)]
struct FField<E: Environment> {
    inner: Field<E>,
}

impl<E: Environment> From<Field<E>> for FField<E> {
    fn from(value: Field<E>) -> Self {
        FField { inner: value }
    }
}

impl<E: Environment> From<FField<E>> for Field<E> {
    fn from(value: FField<E>) -> Self {
        value.inner
    }
}

impl<E: Environment> bincode::Encode for FField<E> {
    fn encode<T: bincode::enc::Encoder>(
        &self,
        encoder: &mut T,
    ) -> core::result::Result<(), bincode::error::EncodeError> {
        bincode::Encode::encode(&self.inner.to_bytes_le().expect("to bytes"), encoder)?;
        Ok(())
    }
}

impl<Context, E: Environment> bincode::Decode<Context> for FField<E> {
    fn decode<D: bincode::de::Decoder<Context = Context>>(
        decoder: &mut D,
    ) -> core::result::Result<Self, bincode::error::DecodeError> {
        let bytes: Vec<u8> = bincode::Decode::decode(decoder)?;
        Ok(FField {
            inner: Field::from_bytes_le(&bytes)
                .map_err(|e| DecodeError::OtherString(e.to_string()))?,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct MerkleProof<E: Environment> {
    root: FField<E>,
    index: usize,
    leaf: FField<E>,
    siblings: Vec<FField<E>>,
}

impl<E: Environment> Encode for MerkleProof<E> {
    fn encode<T: bincode::enc::Encoder>(&self, encoder: &mut T) -> Result<(), EncodeError> {
        self.root.encode(encoder)?;
        self.index.encode(encoder)?;
        self.leaf.encode(encoder)?;
        self.siblings.encode(encoder)?;
        Ok(())
    }
}

impl<Context, E: Environment> Decode<Context> for MerkleProof<E> {
    fn decode<D: bincode::de::Decoder<Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        Ok(MerkleProof {
            root: FField::decode(decoder)?,
            index: usize::decode(decoder)?,
            leaf: FField::decode(decoder)?,
            siblings: Vec::<FField<E>>::decode(decoder)?,
        })
    }
}

pub fn create_proof<E: Environment + Network, const LEVELS: u32>(index: usize) -> MerkleProof<E> {
    let num_leaves = 2u32.pow(LEVELS);
    println!("Creating {} leaves", num_leaves);
    let leaves: Vec<Field<E>> = (0..num_leaves)
        .into_par_iter()
        .map(|i| hash(vec![Field::from_u32(i)]))
        .collect();

    // Create the bottom layer with the leaves
    let mut layers = vec![leaves];
    // We take the layer in pairs and append the hash to the layer above
    while layers.last().unwrap().len() > 1 {
        let prev = layers.last().unwrap();
        let next: Vec<Field<E>> = prev.chunks(2).map(|p| hash(vec![p[0], p[1]])).collect();
        layers.push(next);
    }

    let root = layers.last().unwrap()[0].into();
    let leaf = layers[0][index].into();
    let mut siblings: Vec<FField<E>> = Vec::new();
    let mut idx = index;
    for level in 0..layers.len() - 1 {
        let sibling_idx = if idx.is_multiple_of(2) { idx + 1 } else { idx - 1 };
        siblings.push(layers[level][sibling_idx].into());
        idx /= 2;
    }

    MerkleProof {
        root,
        index,
        leaf,
        siblings,
    }
}

pub mod utils {
    use bincode::error::EncodeError;

    use super::*;
    pub fn save_proof<E: Environment>(proof: &MerkleProof<E>) -> Result<Vec<u8>, EncodeError> {
        bincode::encode_to_vec(proof, bincode::config::standard())
    }

    pub fn load_proof<E: Environment>(bytes: &[u8]) -> Result<MerkleProof<E>, DecodeError> {
        let (proof, _) = bincode::decode_from_slice(bytes, bincode::config::standard())?;
        Ok(proof)
    }

    use std::fs::File;
    use std::io::{self, Write};
    use std::path::Path;

    pub fn write_proof<E: Environment, P: AsRef<Path>>(
        proof: &MerkleProof<E>,
        path: P,
    ) -> io::Result<()> {
        let mut file = File::create(path)?;
        writeln!(
            file,
            "{{\n  root: {},\n  index: {}u32,\n  leaf: {},",
            proof.root.inner, proof.index, proof.leaf.inner
        )?;
        writeln!(file, "  siblings: [")?;
        for sibling in proof.siblings.iter() {
            writeln!(file, "    {0},", sibling.inner)?;
        }
        writeln!(file, "  ]\n}}")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify() {
        let index = 12;
        let proof = create_proof::<TestnetV0, 4>(index);
        assert!(verify(proof));
    }

    #[test]
    fn test_hash_pair() {
        let left = Field::from_u32(1);
        let right = Field::from_u32(2);
        let _ = hash::<TestnetV0>(vec![left, right]);
    }
}
