#![no_std]

mod smbus;

pub mod config;
pub mod device;
pub mod error;
pub mod register;

#[cfg(feature = "async")]
pub mod device_async;

pub use config::{Config, ConfigBuilder, Milliamps, Milliseconds, Port, StringDescriptor};
pub use device::Usb251xb;
pub use error::{Error, StringDescriptorError};
pub use register::Variant;

#[cfg(feature = "async")]
pub use device_async::Usb251xbAsync;

/// Maximum SMBus clock frequency supported by the USB251xB (Hz).
pub const SMBUS_MAX_FREQUENCY_HZ: u32 = 100_000;
