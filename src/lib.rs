mod config;
mod errors;
mod helpers;
mod implementations;
mod traits;

pub use binver_derive::*;
pub use semver::Version;

pub type WriteResult = Result<(), WriteError>;
pub type ReadResult<T = ()> = Result<T, ReadError>;

pub use self::{
    config::ReadConfig,
    errors::{ReadError, WriteError},
    helpers::{deserialize_slice, deserialize_slice_with_config, to_vec, SliceReader},
    traits::{Reader, Serializable, Writer},
};
