use embedded_hal::i2c::I2c;

use crate::error::Error;

/// SMBus block write: sends `[register, byte_count, data...]` as a single I2C write.
pub(crate) fn block_write<I: I2c>(
    i2c: &mut I,
    addr: u8,
    register: u8,
    data: &[u8],
) -> Result<(), Error<I::Error>> {
    let len = data.len() as u8;
    // Max payload: 1 (register) + 1 (count) + 32 (data) = 34
    let mut payload = [0u8; 34];
    payload[0] = register;
    payload[1] = len;
    payload[2..2 + data.len()].copy_from_slice(data);
    i2c.write(addr, &payload[..2 + data.len()])?;
    Ok(())
}

/// SMBus block read: writes the register address, then reads `[byte_count, data...]`.
///
/// Returns the byte count reported by the device.
pub(crate) fn block_read<I: I2c>(
    i2c: &mut I,
    addr: u8,
    register: u8,
    buf: &mut [u8],
) -> Result<u8, Error<I::Error>> {
    i2c.write_read(addr, &[register], buf)?;
    Ok(buf[0])
}

#[cfg(feature = "async")]
pub(crate) mod r#async {
    use embedded_hal_async::i2c::I2c;

    use crate::error::Error;

    pub(crate) async fn block_write<I: I2c>(
        i2c: &mut I,
        addr: u8,
        register: u8,
        data: &[u8],
    ) -> Result<(), Error<I::Error>> {
        let len = data.len() as u8;
        let mut payload = [0u8; 34];
        payload[0] = register;
        payload[1] = len;
        payload[2..2 + data.len()].copy_from_slice(data);
        i2c.write(addr, &payload[..2 + data.len()]).await?;
        Ok(())
    }

    pub(crate) async fn block_read<I: I2c>(
        i2c: &mut I,
        addr: u8,
        register: u8,
        buf: &mut [u8],
    ) -> Result<u8, Error<I::Error>> {
        i2c.write_read(addr, &[register], buf).await?;
        Ok(buf[0])
    }
}
