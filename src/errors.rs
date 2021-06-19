#[derive(Debug)]
pub enum WriteError {
    EndOfOutput,
}

#[derive(Debug)]
pub enum ReadError {
    EndOfInput,
    UnknownVariant(u16),
    #[cfg(feature = "std")]
    InvalidUtf8String(alloc::string::FromUtf8Error),
    InvalidUtf8Str(core::str::Utf8Error),
    TrailingBytes(usize),
}
