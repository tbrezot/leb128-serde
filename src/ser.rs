use crate::Error;
use std::io::Write;

pub struct Serializer {
    output: Vec<u8>,
}

impl Serializer {
    pub const fn new() -> Self {
        Self { output: vec![] }
    }

    pub fn write_u64(&mut self, n: u64) -> Result<usize, Error> {
        leb128::write::unsigned(&mut self.output, n).map_err(|e| {
            Error::InvalidSize(format!(
                "Serializer: unexpected LEB128 error writing {} bytes: {}",
                n, e
            ))
        })
    }

    #[inline]
    pub fn write_usize(&mut self, n: usize) -> Result<usize, Error> {
        self.write_u64(n as u64)
    }

    pub fn write_array(&mut self, array: &[u8]) -> Result<usize, Error> {
        self.output.write(array).map_err(|e| {
            Error::InvalidSize(format!(
                "Serializer: unexpected error writing {} bytes: {}",
                array.len(),
                e
            ))
        })
    }

    pub fn write_vec(&mut self, array: &[u8]) -> Result<usize, Error> {
        let mut len = self.write_u64(array.len() as u64)?;
        len += self.write_array(array)?;
        Ok(len)
    }

    #[inline]
    pub fn value(&self) -> &[u8] {
        &self.output
    }

    #[must_use]
    #[inline]
    pub fn finalize(self) -> Vec<u8> {
        self.output
    }
}

impl Default for Serializer {
    fn default() -> Self {
        Self::new()
    }
}
