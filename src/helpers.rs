use crate::{ReadConfig, ReadError, ReadResult, Reader, Serializable, WriteResult, Writer};
use semver::Version;

pub fn to_vec<T: Serializable>(t: &T) -> Vec<u8> {
    let mut writer = Vec::<u8>::new();
    let version = T::highest_version().expect("Could not determine structure version");
    version.serialize(&mut writer).unwrap();
    t.serialize(&mut writer).unwrap();
    writer
}

pub fn deserialize_slice<T: Serializable>(slice: &[u8]) -> ReadResult<T> {
    deserialize_slice_with_config(slice, ReadConfig::default())
}

pub fn deserialize_slice_with_config<T: Serializable>(
    slice: &[u8],
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

impl<'a> Reader for SliceReader<'a> {
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
}
