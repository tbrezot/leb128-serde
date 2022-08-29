use std::collections::{HashMap, HashSet};

use super::bytes::{Deserializer, Serializer};
use crate::error::Error;

///
/// Serialize vectors of bytes
pub fn serialize_bytes_vectors<T: AsRef<[u8]>>(vectors: &[T]) -> Result<Vec<u8>, Error> {
    let mut serializer = Serializer::new();
    serializer.write_u64(vectors.len() as u64)?;
    for vector in vectors {
        serializer.write_array(vector.as_ref())?;
    }
    Ok(serializer.value().to_vec())
}

///
/// Deserialize vectors of bytes
pub fn deserialize_bytes_vectors(serialized_vectors: &[u8]) -> Result<Vec<Vec<u8>>, Error> {
    let mut de = Deserializer::new(serialized_vectors);
    let len = de.read_u64()? as usize;
    let mut deserialized_vectors = Vec::with_capacity(len);
    for _ in 0..len {
        deserialized_vectors.push(de.read_array()?);
    }
    Ok(deserialized_vectors)
}

///
/// Serialize a `HashMap` of bytes
pub fn serialize_bytes_hashmap(hashmap: &HashMap<Vec<u8>, Vec<u8>>) -> Result<Vec<u8>, Error> {
    let mut serializer = Serializer::new();
    serializer.write_u64(hashmap.len() as u64)?;
    for (key, value) in hashmap {
        serializer.write_array(key)?;
        serializer.write_array(value)?;
    }
    Ok(serializer.value().to_vec())
}

///
/// Deserialize a `HashMap` of bytes
pub fn deserialize_bytes_hashmap(
    serialized_hashmap: &[u8],
) -> Result<HashMap<Vec<u8>, Vec<u8>>, Error> {
    let mut de = Deserializer::new(serialized_hashmap);
    let len = de.read_u64()? as usize;
    let mut output = HashMap::with_capacity(len);
    for _ in 0..len {
        let key = de.read_array()?;
        let value = de.read_array()?;
        output.insert(key, value);
    }
    Ok(output)
}

pub fn serialize_bytes_hashset(hashset: &HashSet<Vec<u8>>) -> Result<Vec<u8>, Error> {
    let mut serializer = Serializer::new();
    serializer.write_u64(hashset.len() as u64)?;
    for hash in hashset {
        serializer.write_array(hash)?;
    }
    Ok(serializer.value().to_vec())
}

///
/// Deserialize a `HashMap` of bytes
pub fn deserialize_bytes_hashset(serialized_hashset: &[u8]) -> Result<HashSet<Vec<u8>>, Error> {
    let mut de = Deserializer::new(serialized_hashset);
    let len = de.read_u64()? as usize;
    let mut output = HashSet::with_capacity(len);
    for _ in 0..len {
        output.insert(de.read_array()?);
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_ser_de_vectors() -> Result<(), Error> {
        let bytes_vectors = vec![vec![1_u8, 2, 3, 4], vec![5, 6, 7, 8, 9]];
        let serialized_bytes_vectors = serialize_bytes_vectors(&bytes_vectors[..])?;

        let deserialized_bytes_vectors = deserialize_bytes_vectors(&serialized_bytes_vectors)?;
        assert_eq!(bytes_vectors, deserialized_bytes_vectors);
        Ok(())
    }

    #[test]
    fn test_ser_de_hashmap() -> Result<(), Error> {
        let mut hashmap = HashMap::new();
        hashmap.insert(vec![1, 2, 3, 4], vec![5, 6, 7, 8]);
        let serialized_hashmap = serialize_bytes_hashmap(&hashmap)?;

        let deserialized_hashmap = deserialize_bytes_hashmap(&serialized_hashmap)?;
        assert_eq!(hashmap, deserialized_hashmap);
        Ok(())
    }

    #[test]
    fn test_ser_de_hashset() -> Result<(), Error> {
        let mut hashset = HashSet::new();
        hashset.insert(vec![1, 2, 3, 4]);
        hashset.insert(vec![5, 6, 7, 8]);
        let serialized_hashset = serialize_bytes_hashset(&hashset)?;

        let deserialized_hashset = deserialize_bytes_hashset(&serialized_hashset)?;
        assert_eq!(hashset, deserialized_hashset);
        Ok(())
    }
}
