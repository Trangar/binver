//! Binary (de)serialization framework that is backwards compatible with versioned fields.
//!
//! ```rust
//! # use binver::*;
//! # #[cfg(feature = "std")]
//! # {
//!
//! #[derive(Serializable, PartialEq, Debug)]
//! pub struct Player {
//!     // This field has existed since binary version 0.0.1
//!     #[since(0.0.1)]
//!     pub id: u32,
//!
//!     // In 0.0.2 we introduced a new field
//!     // When loading a serialized 0.0.1 object, this field will have it's `Default` value
//!     #[since(0.0.2)]
//!     pub name: String,
//! }
//!
//! let player = Player {
//!     id: 5,
//!     name: String::from("foo")
//! };
//!
//! let serialized = binver::to_vec(&player);
//! let deserialized_player = binver::deserialize_slice(&serialized).unwrap();
//!
//! assert_eq!(player, deserialized_player);
//! # }
//! ```

#![no_std]
#![warn(missing_docs)]

#[cfg(feature = "std")]
extern crate alloc;

mod config;
mod errors;
mod helpers;
mod implementations;
mod traits;

pub use binver_derive::*;
pub use semver::Version;

/// Result type to be returned from any write action
pub type WriteResult<T = ()> = Result<T, WriteError>;
/// Result type to be returned from any read action
pub type ReadResult<T = ()> = Result<T, ReadError>;

pub use self::{
    config::ReadConfig,
    errors::{ReadError, WriteError},
    helpers::{deserialize_slice, deserialize_slice_with_config, write_to_slice},
    traits::{Reader, Serializable, Writer},
};

#[cfg(feature = "std")]
pub use self::helpers::to_vec;

lazy_static::lazy_static! {
    #[doc(hidden)]
    pub static ref VERSION: Version = Version::parse(env!("CARGO_PKG_VERSION")).expect("Could not parse cargo package version");
}
