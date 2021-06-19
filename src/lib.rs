#![no_std]

#[cfg(feature = "std")]
extern crate alloc;

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
    helpers::{
        deserialize_slice, deserialize_slice_with_config, write_to_slice, SliceReader,
    },
    traits::{Reader, Serializable, Writer},
};

#[cfg(feature = "std")]
pub use self::helpers::to_vec; 

lazy_static::lazy_static! {
    #[doc(hidden)]
    pub static ref VERSION: Version = Version::parse(env!("CARGO_PKG_VERSION")).expect("Could not parse cargo package version");
}
