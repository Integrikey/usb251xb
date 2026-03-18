# Rust USB251xB/xBi USB 2.0 Hub Controller Driver

`no_std` Rust driver for the Microchip USB2512B, USB2513B and USB2514B
USB 2.0 hub controllers, based on the [`embedded-hal`] traits.

This driver allows you to:
- Configure all hub registers over SMBus/I2C. See `Usb251xb::configure()`.
- Attach the hub to the USB bus. See `Usb251xb::attach()`.

Both blocking (`embedded-hal`) and async (`embedded-hal-async`) interfaces
are provided.

## The Device

The USB251xB/xBi family are USB 2.0 compliant hub controllers with 2, 3 or
4 downstream ports. Configuration is performed over SMBus (up to 100 kHz)
before attaching to the USB bus. The devices support compound device mode,
per-port power switching, battery charging, and configurable USB string
descriptors.

Datasheet: [USB251xB/xBi](https://ww1.microchip.com/downloads/en/devicedoc/00001692c.pdf)

## Usage

### Builder API (recommended)

```rust
use usb251xb::{Config, Port, Usb251xb, Variant};

let config = Config::builder(Variant::Usb2514b)
    .manufacturer("Acme Corp.")
    .expect("encode manufacturer string")
    .compound(true)
    .non_removable_ports(&[Port::Port1])
    .disabled_ports(&[Port::Port4])
    .into_config();

let mut hub = Usb251xb::new(i2c);
hub.configure_and_attach(&config)?;
```

### Async

Enable the `async` feature and use `Usb251xbAsync`:

```rust
use usb251xb::{Config, Usb251xbAsync, Variant};

let config = Config::builder(Variant::Usb2514b).into_config();
let mut hub = Usb251xbAsync::new(i2c);
hub.configure_and_attach(&config).await?;
```

## Features

| Feature | Description |
|---------|-------------|
| `async` | Enables `Usb251xbAsync` via `embedded-hal-async` |
| `defmt` | Derives `defmt::Format` on error types |

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
