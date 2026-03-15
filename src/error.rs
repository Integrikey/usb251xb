use embedded_hal::i2c;

#[derive(Debug, thiserror::Error)]
pub enum Error<E: i2c::Error> {
    /// I2C bus error from the underlying transport.
    #[error("I2C error: {0:?}")]
    I2c(#[from] E),
    /// String descriptor exceeds the 31-codeunit maximum.
    #[error("string too long: {len} codeunits, max {max}")]
    StringTooLong { len: usize, max: usize },
    /// SMBus block read returned an unexpected byte count.
    #[error("unexpected byte count: expected {expected}, got {got}")]
    UnexpectedByteCount { expected: u8, got: u8 },
}
