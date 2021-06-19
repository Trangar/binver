use crate::{
    ReadConfig, ReadError, ReadResult, Reader, Serializable, WriteError, WriteResult, Writer,
};
#[cfg(feature = "std")]
use alloc::vec::Vec;
use semver::Version;

#[cfg(feature = "std")]
pub fn to_vec<'a, T: Serializable<'a>>(t: &T) -> Vec<u8> {
    let mut writer = Vec::<u8>::new();
    crate::VERSION.serialize(&mut writer).unwrap();
    t.serialize(&mut writer).unwrap();
    writer
}

pub fn write_to_slice<'a, T: Serializable<'a>>(slice: &mut [u8], t: &T) -> usize {
    let mut writer = SliceWriter { slice, index: 0 };
    crate::VERSION.serialize(&mut writer).unwrap();
    t.serialize(&mut writer).unwrap();

    writer.index
}

pub fn deserialize_slice<'a, T: Serializable<'a>>(slice: &'a [u8]) -> ReadResult<T> {
    deserialize_slice_with_config(slice, ReadConfig::default())
}

pub fn deserialize_slice_with_config<'a, T: Serializable<'a>>(
    slice: &'a [u8],
    config: ReadConfig,
) -> ReadResult<T> {
    let mut reader = SliceReader {
        version: Version::new(0, 0, 0),
        slice,
        index: 0,
    };
    let version = Version::deserialize(&mut reader)?;
    reader.version = version;
    let result = T::deserialize(&mut reader)?;

    if config.error_on_trailing_bytes && reader.index != reader.slice.len() {
        return Err(ReadError::TrailingBytes(reader.slice.len() - reader.index));
    }
    Ok(result)
}

#[cfg(feature = "std")]
impl Writer for Vec<u8> {
    fn write(&mut self, bytes: &[u8]) -> WriteResult {
        self.extend_from_slice(bytes);
        Ok(())
    }
}

pub struct SliceReader<'a> {
    version: Version,
    slice: &'a [u8],
    index: usize,
}

impl<'a> Reader<'a> for SliceReader<'a> {
    fn version(&self) -> Version {
        self.version.clone()
    }

    fn read(&mut self, bytes: &mut [u8]) -> ReadResult {
        if self.slice.len() < self.index + bytes.len() {
            Err(ReadError::EndOfInput)
        } else {
            bytes.copy_from_slice(&self.slice[self.index..self.index + bytes.len()]);
            self.index += bytes.len();
            Ok(())
        }
    }

    fn read_slice(&mut self, len: usize) -> ReadResult<&'a [u8]> {
        if self.slice.len() < self.index + len {
            Err(ReadError::EndOfInput)
        } else {
            let slice = &self.slice[self.index..][..len];
            self.index += len;
            Ok(slice)
        }
    }
}
pub struct SliceWriter<'a> {
    slice: &'a mut [u8],
    index: usize,
}

impl<'a> Writer for SliceWriter<'a> {
    fn write(&mut self, bytes: &[u8]) -> WriteResult {
        if self.slice.len() < self.index + bytes.len() {
            Err(WriteError::EndOfOutput)
        } else {
            self.slice[self.index..self.index + bytes.len()].copy_from_slice(bytes);
            self.index += bytes.len();
            Ok(())
        }
    }
}
