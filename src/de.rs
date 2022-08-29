//! Implement the `Serializer` and `Deserializer` objects using LEB128.

use crate::Error;
use generic_array::{ArrayLength, GenericArray};
use std::io::Read;

pub struct Deserializer<'a> {
    input: &'a [u8],
}

impl<'a> Deserializer<'a> {
    pub const fn new(bytes: &'a [u8]) -> Deserializer<'a> {
        Deserializer { input: bytes }
    }

    pub fn read_u64(&mut self) -> Result<u64, Error> {
        leb128::read::unsigned(&mut self.input).map_err(|e| {
            Error::InvalidSize(format!(
                "Deserializer: failed reading the size of the next array: {}",
                e
            ))
        })
    }

    pub fn read_usize(&mut self) -> Result<usize, Error> {
        let len_u64 = self.read_u64()?;
        usize::try_from(len_u64).map_err(|_| {
            Error::InvalidSize(format!(
                "Deserializer: size of array is too big: {} bytes",
                len_u64
            ))
        })
    }

    pub fn read_vec(&mut self) -> Result<Vec<u8>, Error> {
        let len = self.read_usize()?;
        let mut buf = vec![0; len];
        if len != 0 {
            self.input.read_exact(&mut buf).map_err(|_| {
                Error::InvalidSize(format!(
                    "Deserializer: failed reading array of: {} bytes",
                    len
                ))
            })?;
        }
        Ok(buf)
    }

    pub fn read_array<Length: ArrayLength<u8>>(
        &mut self,
    ) -> Result<GenericArray<u8, Length>, Error> {
        let mut buf = GenericArray::<u8, Length>::default();
        self.input.read_exact(&mut buf).map_err(|_| {
            Error::InvalidSize(format!(
                "Deserializer: failed reading array of: {} bytes",
                Length::to_usize()
            ))
        })?;
        Ok(buf)
    }
}
