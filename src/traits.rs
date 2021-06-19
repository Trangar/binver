use crate::{ReadResult, WriteResult};
use semver::Version;

pub trait Serializable: Sized {
    fn serialize(&self, writer: &mut dyn Writer) -> WriteResult;
    fn deserialize(reader: &mut dyn Reader) -> ReadResult<Self>;
}

pub trait Writer {
    fn write(&mut self, bytes: &[u8]) -> WriteResult;
}

pub trait Reader {
    fn version(&self) -> Version;
    fn read(&mut self, bytes: &mut [u8]) -> ReadResult;
}
