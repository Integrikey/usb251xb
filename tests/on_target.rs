#![no_std]
#![no_main]

use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::i2c::{self, I2c};
use embassy_rp::peripherals::I2C1;
use embassy_time::Timer;
use usb251xb::config::StringDescriptor;
use usb251xb::register::*;
use usb251xb::{Config, Usb251xbAsync, Variant};

bind_interrupts!(struct Irqs {
    I2C1_IRQ => i2c::InterruptHandler<I2C1>;
});

#[embedded_test::setup]
fn setup() {
    rtt_target::rtt_init_defmt!();
}

#[embedded_test::tests]
mod tests {
    use super::*;

    struct State {
        hub: Usb251xbAsync<I2c<'static, embassy_rp::peripherals::I2C1, i2c::Async>>,
        _reset: Output<'static>,
    }

    #[init]
    async fn init() -> State {
        let p = embassy_rp::init(Default::default());

        // Assert RESET_N low, wait, then release to enter SMBus slave mode.
        let mut reset = Output::new(p.PIN_1, Level::Low);
        Timer::after_millis(10).await;
        reset.set_high();
        // Datasheet: allow at least 3ms after reset for SMBus to be ready.
        Timer::after_millis(5).await;

        let mut i2c_config = i2c::Config::default();
        i2c_config.frequency = 100_000;

        let i2c = I2c::new_async(p.I2C1, p.PIN_3, p.PIN_2, Irqs, i2c_config);
        let hub = Usb251xbAsync::new(i2c);

        State { hub, _reset: reset }
    }

    #[test]
    async fn configure_default(mut state: State) {
        let config = Config::for_variant(Variant::Usb2514b);
        state
            .hub
            .configure(&config)
            .await
            .expect("configure with defaults");
    }

    #[test]
    async fn configure_and_attach(mut state: State) {
        let config = Config::for_variant(Variant::Usb2514b);
        state
            .hub
            .configure_and_attach(&config)
            .await
            .expect("configure and attach");
    }

    #[test]
    async fn read_vendor_id_after_configure(mut state: State) {
        let config = Config::for_variant(Variant::Usb2514b);
        state
            .hub
            .configure(&config)
            .await
            .expect("configure before read");

        // SMBus block read: buf[0] = byte count, buf[1..] = data.
        // The device returns 32 bytes from the start register address.
        let mut buf = [0u8; 33];
        let count = state
            .hub
            .read_register(REG_VENDOR_ID_LSB, &mut buf)
            .await
            .expect("read registers");
        defmt::assert_eq!(count, 32, "expected 32-byte block read");
        let vendor_id = u16::from_le_bytes([buf[1], buf[2]]);
        defmt::assert_eq!(vendor_id, 0x0424, "expected Microchip vendor ID");
    }

    #[test]
    async fn manufacturer_string_roundtrip(mut state: State) {
        let mut config = Config::for_variant(Variant::Usb2514b);
        config.manufacturer_string =
            StringDescriptor::encode("Keystrike Inc.").expect("encode manufacturer string");
        state
            .hub
            .configure(&config)
            .await
            .expect("configure with manufacturer string");

        // Read the string length register (0x13) to confirm byte count.
        let mut len_buf = [0u8; 33];
        let _count = state
            .hub
            .read_register(REG_MANUFACTURER_STRING_LEN, &mut len_buf)
            .await
            .expect("read string length register");
        // "Keystrike Inc." = 14 codeunits * 2 = 28 bytes
        defmt::assert_eq!(len_buf[1], 28, "manufacturer string byte length");

        // Read the string data (0x16). 28 bytes fits in one 32-byte block.
        let mut str_buf = [0u8; 33];
        let count = state
            .hub
            .read_register(REG_MANUFACTURER_STRING, &mut str_buf)
            .await
            .expect("read manufacturer string");
        defmt::assert_eq!(count, 32, "expected 32-byte block read");

        // Decode UTF-16LE from str_buf[1..29] and compare.
        let mut expected = [0u8; 28];
        let mut i = 0;
        for c in "Keystrike Inc.".encode_utf16() {
            let le = c.to_le_bytes();
            expected[i] = le[0];
            expected[i + 1] = le[1];
            i += 2;
        }
        defmt::assert_eq!(str_buf[1..29], expected[..], "manufacturer string data");
    }
}
