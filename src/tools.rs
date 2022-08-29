use crate::{Deserializer, Error, Serializer};
use std::collections::{HashMap, HashSet};

///
/// Serialize vectors of bytes
pub fn serialize_bytes_vectors<T: AsRef<[u8]>>(vectors: &[T]) -> Result<Vec<u8>, Error> {
    let mut serializer = Serializer::new();
    serializer.write_usize(vectors.len())?;
    for vector in vectors {
        serializer.write_vec(vector.as_ref())?;
    }
    Ok(serializer.finalize())
}

///
/// Deserialize vectors of bytes
pub fn deserialize_bytes_vectors(serialized_vectors: &[u8]) -> Result<Vec<Vec<u8>>, Error> {
    let mut de = Deserializer::new(serialized_vectors);
    let len = de.read_usize()?;
    let mut deserialized_vectors = Vec::with_capacity(len);
    for _ in 0..len {
        deserialized_vectors.push(de.read_vec()?);
    }
    Ok(deserialized_vectors)
}

///
/// Serialize a `HashMap` of bytes
pub fn serialize_bytes_hashmap(hashmap: &HashMap<Vec<u8>, Vec<u8>>) -> Result<Vec<u8>, Error> {
    let mut serializer = Serializer::new();
    serializer.write_usize(hashmap.len())?;
    for (key, value) in hashmap {
        serializer.write_vec(key)?;
        serializer.write_vec(value)?;
    }
    Ok(serializer.finalize())
}

///
/// Deserialize a `HashMap` of bytes
pub fn deserialize_bytes_hashmap(
    serialized_hashmap: &[u8],
) -> Result<HashMap<Vec<u8>, Vec<u8>>, Error> {
    let mut de = Deserializer::new(serialized_hashmap);
    let len = de.read_usize()?;
    let mut output = HashMap::with_capacity(len);
    for _ in 0..len {
        let key = de.read_vec()?;
        let value = de.read_vec()?;
        output.insert(key, value);
    }
    Ok(output)
}

pub fn serialize_bytes_hashset(hashset: &HashSet<Vec<u8>>) -> Result<Vec<u8>, Error> {
    let mut serializer = Serializer::new();
    serializer.write_usize(hashset.len())?;
    for hash in hashset {
        serializer.write_vec(hash)?;
    }
    Ok(serializer.finalize())
}

///
/// Deserialize a `HashMap` of bytes
pub fn deserialize_bytes_hashset(serialized_hashset: &[u8]) -> Result<HashSet<Vec<u8>>, Error> {
    let mut de = Deserializer::new(serialized_hashset);
    let len = de.read_usize()?;
    let mut output = HashSet::with_capacity(len);
    for _ in 0..len {
        output.insert(de.read_vec()?);
    }
    Ok(output)
}
