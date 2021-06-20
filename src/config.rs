/// Configuration passed to a read function
#[derive(Default, Clone)]
pub struct ReadConfig {
    /// Set this to `true` to make the serialize functions return `ReadError::TrailingBytes` if the reader is not empty.
    pub error_on_trailing_bytes: bool,
}
