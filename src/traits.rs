use crate::{ReadResult, Version, WriteResult};

/// The main trait of this crate that is used for (de)serialization
pub trait Serializable<'a>: Sized {
    /// Serialize the current object into the given writer.
    fn serialize(&self, writer: &mut dyn Writer) -> WriteResult;

    /// Attempt to deserialize this object from the given reader.
    fn deserialize(reader: &mut dyn Reader<'a>) -> ReadResult<Self>;
}

/// Generic writer
pub trait Writer {
    /// Write all bytes from the given slice to the writer.
    /// This function must block until all bytes have been written.
    fn write(&mut self, bytes: &[u8]) -> WriteResult;
}

/// Generic reader
pub trait Reader<'a> {
    /// Return the version of the format being read.
    fn version(&self) -> Version;
    /// Fill the given slice with bytes. All bytes must be read.
    fn read(&mut self, bytes: &mut [u8]) -> ReadResult;
    /// Read a slice of length `len` from the reader. Must be return exactly the amount of bytes being requested.
    ///
    /// If the reader does not have it's own internal buffer (e.g. `std::fs::File`) `ReadError::ReaderNotPersistent` should be returned.
    fn read_slice(&mut self, len: usize) -> ReadResult<&'a [u8]>;
}
