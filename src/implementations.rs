use crate::{ReadError, ReadResult, Reader, Serializable, WriteResult, Writer};
use semver::Version;

impl Serializable for Version {
    fn serialize(&self, writer: &mut dyn Writer) -> WriteResult {
        (self.major as u16).serialize(writer)?;
        (self.minor as u16).serialize(writer)?;
        (self.patch as u16).serialize(writer)?;
        Ok(())
    }
    fn deserialize(reader: &mut dyn Reader) -> ReadResult<Self> {
        let major = u16::deserialize(reader)?;
        let minor = u16::deserialize(reader)?;
        let patch = u16::deserialize(reader)?;
        Ok(Version::new(major as u64, minor as u64, patch as u64))
    }
}

impl Serializable for String {
    fn serialize(&self, writer: &mut dyn Writer) -> WriteResult {
        (self.len() as u32).serialize(writer)?;
        writer.write(self.as_bytes())?;
        Ok(())
    }
    fn deserialize(reader: &mut dyn Reader) -> ReadResult<Self> {
        let len = u32::deserialize(reader)? as usize;
        let mut blob = vec![0u8; len];
        reader.read(&mut blob[..len])?;
        String::from_utf8(blob).map_err(ReadError::InvalidUtf8String)
    }
}

impl<T: Serializable> Serializable for Vec<T> {
    fn serialize(&self, writer: &mut dyn Writer) -> WriteResult {
        (self.len() as u32).serialize(writer)?;
        for item in self.iter() {
            item.serialize(writer)?;
        }
        Ok(())
    }
    fn deserialize(reader: &mut dyn Reader) -> ReadResult<Self> {
        let len = u32::deserialize(reader)? as usize;
        let mut blob = Vec::with_capacity(len);
        for _ in 0..len {
            blob.push(T::deserialize(reader)?);
        }
        Ok(blob)
    }
}

macro_rules! impl_numeric {
    ($($ty:ty),*) => {
        $(
            impl Serializable for $ty {
                fn serialize(&self, writer: &mut dyn Writer) -> WriteResult {
                    let bytes = self.to_be_bytes();
                    writer.write(&bytes)
                }

                fn deserialize(reader: &mut dyn Reader) -> ReadResult<Self> {
                    let mut bytes = [0u8; core::mem::size_of::<Self>()];
                    reader.read(&mut bytes[..])?;
                    Ok(Self::from_be_bytes(bytes))
                }
            }
        )*
    }
}

impl_numeric! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize
}
