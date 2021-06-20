/// Error thrown while writing
#[derive(Debug)]
pub enum WriteError {
    /// Could not fit the entire object into the given reader.
    EndOfOutput,
}

/// Error thrown while reading
#[derive(Debug)]
pub enum ReadError {
    /// The reader was exhausted before the object could be deserialized
    EndOfInput,

    /// While reading a variant, encountered a variant that did not exist in the current version.
    UnknownVariant(u16),

    #[cfg(feature = "std")]
    /// While reading a `String`, could not interpret it as a valid UTF8 string
    InvalidUtf8String(alloc::string::FromUtf8Error),

    /// While reading a `&str`, could not interpret it as a valid UTF8 string
    InvalidUtf8Str(core::str::Utf8Error),

    /// If `ReadConfig::error_on_trailing_bytes` is set to `true`, this error will be thrown when there are still bytes left to be read.
    TrailingBytes(usize),

    /// Throws an error when `Reader::read_slice` is called but the reader does not have a persistent buffer.
    ReaderNotPersistent,
}
