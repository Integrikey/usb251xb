use embedded_hal::i2c::I2c;

use crate::config::Config;
use crate::error::Error;
use crate::register::{DEVICE_ADDR, REG_STATUS_COMMAND, StatusCommand};
use crate::smbus;

/// Synchronous driver for the USB251xB/xBi hub controller.
///
/// The underlying I2C bus must be configured at or below 100 kHz
/// (SMBus frequency limit).
pub struct Usb251xb<I> {
    i2c: I,
}

impl<I: I2c> Usb251xb<I> {
    /// Creates a new driver instance, taking ownership of the I2C bus.
    ///
    /// The bus must be configured at or below 100 kHz.
    pub fn new(i2c: I) -> Self {
        Self { i2c }
    }

    /// Writes the full configuration to the hub controller.
    pub fn configure(&mut self, config: &Config) -> Result<(), Error<I::Error>> {
        let chunks = config.to_register_chunks();
        for &(reg, ref data, len) in &chunks {
            if len == 0 {
                continue;
            }
            smbus::block_write(&mut self.i2c, DEVICE_ADDR, reg, &data[..len as usize])?;
        }
        Ok(())
    }

    /// Triggers USB attach so the host enumerates the hub.
    pub fn attach(&mut self) -> Result<(), Error<I::Error>> {
        let cmd = StatusCommand::new().with_usb_attach(true);
        smbus::block_write(
            &mut self.i2c,
            DEVICE_ADDR,
            REG_STATUS_COMMAND,
            &cmd.into_bytes(),
        )
    }

    /// Writes configuration and then triggers USB attach.
    pub fn configure_and_attach(&mut self, config: &Config) -> Result<(), Error<I::Error>> {
        self.configure(config)?;
        self.attach()
    }

    /// Resets the hub controller.
    pub fn reset(&mut self) -> Result<(), Error<I::Error>> {
        let cmd = StatusCommand::new().with_reset(true);
        smbus::block_write(
            &mut self.i2c,
            DEVICE_ADDR,
            REG_STATUS_COMMAND,
            &cmd.into_bytes(),
        )
    }

    /// Reads a register via SMBus block read. Returns the byte count from the device.
    pub fn read_register(&mut self, register: u8, buf: &mut [u8]) -> Result<u8, Error<I::Error>> {
        smbus::block_read(&mut self.i2c, DEVICE_ADDR, register, buf)
    }

    /// Releases the I2C bus, consuming the driver.
    pub fn release(self) -> I {
        self.i2c
    }
}
