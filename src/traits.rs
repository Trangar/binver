use crate::{ReadResult, WriteResult};
use semver::Version;

pub trait Serializable<'a>: Sized {
    fn serialize(&self, writer: &mut dyn Writer) -> WriteResult;
    fn deserialize(reader: &mut dyn Reader<'a>) -> ReadResult<Self>;
}

pub trait Writer {
    fn write(&mut self, bytes: &[u8]) -> WriteResult;
}

pub trait Reader<'a> {
    fn version(&self) -> Version;
    fn read(&mut self, bytes: &mut [u8]) -> ReadResult;
    fn read_slice(&mut self, len: usize) -> ReadResult<&'a [u8]>;
}
