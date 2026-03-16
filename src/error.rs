use embedded_hal::i2c;

/// Error returned by [`StringDescriptor`](crate::config::StringDescriptor) operations.
#[derive(Debug, thiserror::Error)]
pub enum StringDescriptorError {
    /// The string exceeds the 31-codeunit maximum.
    #[error("string too long: {len} codeunits, max {max}")]
    TooLong { len: usize, max: usize },
}

/// Error type for USB251xB driver operations.
#[derive(Debug, thiserror::Error)]
pub enum Error<E: i2c::Error> {
    /// I2C bus error from the underlying transport.
    #[error("I2C error: {0:?}")]
    I2c(#[from] E),
    /// Invalid string descriptor.
    #[error(transparent)]
    StringDescriptor(#[from] StringDescriptorError),
    /// SMBus block read returned an unexpected byte count.
    #[error("unexpected byte count: expected {expected}, got {got}")]
    UnexpectedByteCount { expected: u8, got: u8 },
}
