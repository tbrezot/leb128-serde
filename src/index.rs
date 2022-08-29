use std::collections::{HashMap, HashSet};

use super::bytes::{Deserializer, Serializer};
use crate::error::Error;

///
/// Serialize vectors of bytes
pub fn serialize_bytes_vectors<T: AsRef<[u8]>>(vectors: &[T]) -> Result<Vec<u8>, Error> {
    let mut serializer = Serializer::new();
    for vector in vectors {
        serializer.write_array(vector.as_ref())?;
    }
    // empty array marks the end
    serializer.write_array(&[])?;
    Ok(serializer.value().to_vec())
}

///
/// Deserialize vectors of bytes
pub fn deserialize_bytes_vectors(serialized_vectors: &[u8]) -> Result<Vec<Vec<u8>>, Error> {
    let mut deserialized_vectors = Vec::new();
    let mut de = Deserializer::new(serialized_vectors);
    loop {
        let bytes = de.read_array()?;
        if bytes.is_empty() {
            // empty array marks the end
            break;
        }
        deserialized_vectors.push(bytes);
    }
    Ok(deserialized_vectors)
}

///
/// Serialize a `HashMap` of bytes
#[allow(dead_code)]
pub fn serialize_bytes_hashmap(hashmap: &HashMap<Vec<u8>, Vec<u8>>) -> Result<Vec<u8>, Error> {
    let mut serializer = Serializer::new();
    for (key, value) in hashmap {
        serializer.write_array(key)?;
        serializer.write_array(value)?;
    }
    // empty array marks the end
    serializer.write_array(&[])?;

    Ok(serializer.value().to_vec())
}

///
/// Deserialize a `HashMap` of bytes
pub fn deserialize_bytes_hashmap(
    serialized_hashmap: &[u8],
) -> Result<HashMap<Vec<u8>, Vec<u8>>, Error> {
    if serialized_hashmap.is_empty() {
        return Ok(HashMap::new());
    }

    let mut output_hashmap = HashMap::new();
    let mut de = Deserializer::new(serialized_hashmap);
    loop {
        let key = de.read_array()?;
        if key.is_empty() {
            // empty array marks the end
            break;
        }
        let value = de.read_array()?;
        output_hashmap.insert(key, value);
    }
    Ok(output_hashmap)
}

#[allow(dead_code)]
pub fn serialize_bytes_hash_set(hashset: &HashSet<Vec<u8>>) -> Result<Vec<u8>, Error> {
    let mut serializer = Serializer::new();
    for hash in hashset {
        serializer.write_array(hash)?;
    }
    // write an empty array to mak the end (wastes one byte)
    serializer.write_array(&[])?;
    Ok(serializer.value().to_vec())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{deserialize_bytes_vectors, serialize_bytes_vectors};
    use crate::{
        error::Error,
        index::{deserialize_bytes_hashmap, serialize_bytes_hashmap},
    };

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
}
