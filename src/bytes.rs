//! Implement the `Serializer` and `Deserializer` objects using LEB128.

use crate::Error;
use std::io::{Read, Write};

pub struct Deserializer<'a> {
    readable: &'a [u8],
}

impl<'a> Deserializer<'a> {
    pub const fn new(bytes: &'a [u8]) -> Deserializer<'a> {
        Deserializer { readable: bytes }
    }

    pub fn read_u64(&mut self) -> Result<u64, Error> {
        leb128::read::unsigned(&mut self.readable).map_err(|e| {
            Error::InvalidSize(format!(
                "Deserializer: failed reading the size of the next array: {}",
                e
            ))
        })
    }

    pub fn read_array(&mut self) -> Result<Vec<u8>, Error> {
        let len_u64 = self.read_u64()?;
        if len_u64 == 0 {
            return Ok(vec![]);
        };
        let len = usize::try_from(len_u64).map_err(|_| {
            Error::InvalidSize(format!(
                "Deserializer: size of array is too big: {} bytes",
                len_u64
            ))
        })?;
        let mut buf = vec![0_u8; len];
        self.readable.read_exact(&mut buf).map_err(|_| {
            Error::InvalidSize(format!(
                "Deserializer: failed reading array of: {} bytes",
                len
            ))
        })?;
        Ok(buf)
    }
}

pub struct Serializer {
    writable: Vec<u8>,
}

impl Serializer {
    pub const fn new() -> Self {
        Self { writable: vec![] }
    }

    pub fn write_u64(&mut self, n: u64) -> Result<usize, Error> {
        leb128::write::unsigned(&mut self.writable, n as u64).map_err(|e| {
            Error::InvalidSize(format!(
                "Serializer: unexpected LEB128 error writing {} bytes: {}",
                n, e
            ))
        })
    }

    pub fn write_array(&mut self, array: &[u8]) -> Result<usize, Error> {
        let mut len = self.write_u64(array.len() as u64)?;
        len += self.writable.write(array).map_err(|e| {
            Error::InvalidSize(format!(
                "Serializer: unexpected error writing {} bytes: {}",
                array.len(),
                e
            ))
        })?;
        Ok(len)
    }

    pub fn value(&self) -> &[u8] {
        &self.writable
    }
}

impl Default for Serializer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Deserializer, Serializer};
    use crate::error::Error;

    #[test]
    pub fn test_ser_de() -> Result<(), Error> {
        let a1 = b"azerty".to_vec();
        let a2 = b"".to_vec();
        let a3 = "nbvcxwmlkjhgfdsqpoiuytreza)????_??-('??&".as_bytes().to_vec();

        let mut ser = Serializer::new();
        assert_eq!(7, ser.write_array(&a1)?);
        assert_eq!(1, ser.write_array(&a2)?);
        assert_eq!(41, ser.write_array(&a3)?);
        assert_eq!(49, ser.value().len());

        let mut de = Deserializer::new(ser.value());
        let a1_ = de.read_array()?;
        assert_eq!(a1, a1_);
        let a2_ = de.read_array()?;
        assert_eq!(a2, a2_);
        let a3_ = de.read_array()?;
        assert_eq!(a3, a3_);

        Ok(())
    }
}
