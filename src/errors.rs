#[derive(Debug)]
pub enum WriteError {}

#[derive(Debug)]
pub enum ReadError {
    EndOfInput,
    UnknownVariant(u16),
    InvalidUtf8String(std::string::FromUtf8Error),
    TrailingBytes(usize),
}
