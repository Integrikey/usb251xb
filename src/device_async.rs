use embedded_hal_async::i2c::I2c;

use crate::config::Config;
use crate::error::Error;
use crate::register::{DEVICE_ADDR, REG_STATUS_COMMAND, StatusCommand};
use crate::smbus::r#async as smbus;

/// Async driver for the USB251xB/xBi hub controller.
///
/// The underlying I2C bus must be configured at or below 100 kHz
/// (SMBus frequency limit).
pub struct Usb251xbAsync<I> {
    i2c: I,
}

impl<I: I2c> Usb251xbAsync<I> {
    /// Creates a new async driver instance, taking ownership of the I2C bus.
    ///
    /// The bus must be configured at or below 100 kHz.
    pub fn new(i2c: I) -> Self {
        Self { i2c }
    }

    /// Writes the full configuration to the hub controller.
    pub async fn configure(&mut self, config: &Config) -> Result<(), Error<I::Error>> {
        let chunks = config.to_register_chunks();
        for &(reg, ref data, len) in &chunks {
            if len == 0 {
                continue;
            }
            smbus::block_write(&mut self.i2c, DEVICE_ADDR, reg, &data[..len as usize]).await?;
        }
        Ok(())
    }

    /// Triggers USB attach so the host enumerates the hub.
    pub async fn attach(&mut self) -> Result<(), Error<I::Error>> {
        let cmd = StatusCommand::new().with_usb_attach(true);
        smbus::block_write(
            &mut self.i2c,
            DEVICE_ADDR,
            REG_STATUS_COMMAND,
            &cmd.into_bytes(),
        )
        .await
    }

    /// Writes configuration and then triggers USB attach.
    pub async fn configure_and_attach(&mut self, config: &Config) -> Result<(), Error<I::Error>> {
        self.configure(config).await?;
        self.attach().await
    }

    /// Resets the hub controller.
    pub async fn reset(&mut self) -> Result<(), Error<I::Error>> {
        let cmd = StatusCommand::new().with_reset(true);
        smbus::block_write(
            &mut self.i2c,
            DEVICE_ADDR,
            REG_STATUS_COMMAND,
            &cmd.into_bytes(),
        )
        .await
    }

    /// Reads a register via SMBus block read. Returns the byte count from the device.
    pub async fn read_register(
        &mut self,
        register: u8,
        buf: &mut [u8],
    ) -> Result<u8, Error<I::Error>> {
        smbus::block_read(&mut self.i2c, DEVICE_ADDR, register, buf).await
    }

    /// Releases the I2C bus, consuming the driver.
    pub fn release(self) -> I {
        self.i2c
    }
}
