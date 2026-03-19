//! `no_std` driver for Microchip USB251xB/xBi USB 2.0 hub controllers.
//!
//! Provides register-level configuration over SMBus/I2C and an ergonomic
//! [`ConfigBuilder`] for common setups. Both blocking ([`Usb251xb`]) and
//! async ([`Usb251xbAsync`]) interfaces are available.
//!
//! # Example
//!
//! ```rust,no_run
//! # fn example(i2c: impl embedded_hal::i2c::I2c) -> Result<(), Box<dyn core::error::Error>> {
//! use usb251xb::{Config, Port, Usb251xb, Variant};
//!
//! let config = Config::builder(Variant::Usb2514b)
//!     .manufacturer("Acme Corp.")?
//!     .compound(true)
//!     .non_removable_ports(&[Port::Port1])
//!     .disabled_ports(&[Port::Port4])
//!     .into_config();
//!
//! let mut hub = Usb251xb::new(i2c);
//! hub.configure_and_attach(&config)?;
//! # Ok(())
//! # }
//! ```
//!
//! [Datasheet (PDF)](https://ww1.microchip.com/downloads/en/devicedoc/00001692c.pdf)

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
pub use register::{BoostLevel, CurrentSensing, LogicalPort, OcTimer, PowerSwitching, Variant};

#[cfg(feature = "async")]
pub use device_async::Usb251xbAsync;

/// Maximum SMBus clock frequency supported by the USB251xB (Hz).
pub const SMBUS_MAX_FREQUENCY_HZ: u32 = 100_000;
