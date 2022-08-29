#[cfg(test)]
use crate::error::Error;
use crate::{tools::*, Deserializer, Serializer};
use std::collections::{HashMap, HashSet};

#[test]
pub fn test_ser_de() -> Result<(), Error> {
    let a1 = b"azerty".to_vec();
    let a2 = b"".to_vec();
    let a3 = "nbvcxwmlkjhgfdsqpoiuytreza)àç_è-('é&".as_bytes().to_vec();

    let mut ser = Serializer::new();
    assert_eq!(7, ser.write_vec(&a1)?);
    assert_eq!(1, ser.write_vec(&a2)?);
    assert_eq!(41, ser.write_vec(&a3)?);
    assert_eq!(49, ser.value().len());

    let mut de = Deserializer::new(ser.value());
    let a1_ = de.read_vec()?;
    assert_eq!(a1, a1_);
    let a2_ = de.read_vec()?;
    assert_eq!(a2, a2_);
    let a3_ = de.read_vec()?;
    assert_eq!(a3, a3_);

    Ok(())
}

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
