use crate::error::StringDescriptorError;
use crate::register::*;

/// A single register chunk: (start_register, data_buffer, data_length).
type Chunk = (u8, [u8; 32], u8);

/// Maximum number of UTF-16 code units in a string descriptor.
const MAX_STRING_CODEUNITS: usize = 31;

/// USB string descriptor stored as UTF-16LE with a length prefix.
#[derive(Debug, Clone, Default)]
pub struct StringDescriptor {
    buf: [u16; MAX_STRING_CODEUNITS],
    len: u8,
}

impl StringDescriptor {
    /// Creates an empty string descriptor.
    pub const fn empty() -> Self {
        Self {
            buf: [0u16; MAX_STRING_CODEUNITS],
            len: 0,
        }
    }

    /// Encodes a `&str` into a USB string descriptor (UTF-16LE).
    ///
    /// Returns [`StringDescriptorError::TooLong`] if the string encodes to
    /// more than 31 UTF-16 code units.
    pub fn encode(s: &str) -> Result<Self, StringDescriptorError> {
        let mut buf = [0u16; MAX_STRING_CODEUNITS];
        let mut i = 0;
        for c in s.encode_utf16() {
            if i >= MAX_STRING_CODEUNITS {
                return Err(StringDescriptorError::TooLong {
                    len: s.encode_utf16().count(),
                    max: MAX_STRING_CODEUNITS,
                });
            }
            buf[i] = c;
            i += 1;
        }
        Ok(Self { buf, len: i as u8 })
    }

    /// Number of UTF-16 code units.
    pub const fn len(&self) -> u8 {
        self.len
    }

    /// Whether the descriptor is empty.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Number of bytes when serialized as UTF-16LE (2 bytes per code unit).
    pub const fn byte_len(&self) -> u8 {
        self.len * 2
    }

    /// Writes the UTF-16LE bytes into `dest`, returning the number of bytes written.
    pub fn write_le_bytes(&self, dest: &mut [u8]) -> usize {
        let n = self.len as usize;
        for (i, cu) in self.buf[..n].iter().enumerate() {
            let le = cu.to_le_bytes();
            dest[i * 2] = le[0];
            dest[i * 2 + 1] = le[1];
        }
        n * 2
    }
}

/// Full configuration for a USB251xB/xBi hub controller.
pub struct Config {
    pub vendor_id: u16,
    pub product_id: u16,
    pub device_id: u16,
    pub config1: ConfigByte1,
    pub config2: ConfigByte2,
    pub config3: ConfigByte3,
    pub non_removable: PortBitfield,
    pub port_disable_self: PortBitfield,
    pub port_disable_bus: PortBitfield,
    /// Maximum power draw in self-powered mode (mA). Divided by 2 on serialization.
    pub max_power_self_ma: u16,
    /// Maximum power draw in bus-powered mode (mA). Divided by 2 on serialization.
    pub max_power_bus_ma: u16,
    /// Hub controller current in self-powered mode (mA). Divided by 2 on serialization.
    pub hub_current_self_ma: u16,
    /// Hub controller current in bus-powered mode (mA). Divided by 2 on serialization.
    pub hub_current_bus_ma: u16,
    /// Power-on time (ms). Divided by 2 on serialization.
    pub power_on_time_ms: u16,
    pub language_id: u16,
    pub manufacturer_string: StringDescriptor,
    pub product_string: StringDescriptor,
    pub serial_string: StringDescriptor,
    pub battery_charging: PortBitfield,
    pub boost_upstream: BoostUpstream,
    pub boost_downstream: BoostDownstream,
    pub port_swap: PortBitfield,
    pub port_map_12: PortMap12,
    pub port_map_34: PortMap34,
}

impl Config {
    /// Returns the datasheet-default configuration for the given variant.
    pub fn for_variant(variant: Variant) -> Self {
        Self {
            vendor_id: 0x0424,
            product_id: variant.product_id(),
            device_id: 0x0BB3,
            config1: ConfigByte1::from_bytes([0x9B]),
            config2: ConfigByte2::from_bytes([0x20]),
            config3: ConfigByte3::from_bytes([0x00]),
            non_removable: PortBitfield::new(),
            port_disable_self: PortBitfield::new(),
            port_disable_bus: PortBitfield::new(),
            max_power_self_ma: 2,
            max_power_bus_ma: 100,
            hub_current_self_ma: 100,
            hub_current_bus_ma: 100,
            power_on_time_ms: 100,
            language_id: 0x0409,
            manufacturer_string: StringDescriptor::empty(),
            product_string: StringDescriptor::empty(),
            serial_string: StringDescriptor::empty(),
            battery_charging: PortBitfield::new(),
            boost_upstream: BoostUpstream::new(),
            boost_downstream: BoostDownstream::new(),
            port_swap: PortBitfield::new(),
            port_map_12: PortMap12::new()
                .with_port1(LogicalPort::Port1)
                .with_port2(LogicalPort::Port2),
            port_map_34: PortMap34::new()
                .with_port3(LogicalPort::Port3)
                .with_port4(LogicalPort::Port4),
        }
    }

    /// Serializes the configuration into register chunks for SMBus block writes.
    ///
    /// Returns an array of `(register_address, data_buffer, data_length)` tuples.
    /// Each chunk is at most 32 bytes, fitting within an SMBus block write.
    pub fn to_register_chunks(&self) -> [Chunk; 12] {
        let mut chunks: [Chunk; 12] = [(0, [0u8; 32], 0); 12];

        // Chunk 0: IDs + config bytes + string lengths (0x00-0x15, 22 bytes)
        {
            let buf = &mut chunks[0].1;
            buf[0] = self.vendor_id as u8;
            buf[1] = (self.vendor_id >> 8) as u8;
            buf[2] = self.product_id as u8;
            buf[3] = (self.product_id >> 8) as u8;
            buf[4] = self.device_id as u8;
            buf[5] = (self.device_id >> 8) as u8;
            buf[6] = self.config1.into_bytes()[0];
            buf[7] = self.config2.into_bytes()[0];
            buf[8] = self.config3.into_bytes()[0];
            buf[9] = self.non_removable.into_bytes()[0];
            buf[10] = self.port_disable_self.into_bytes()[0];
            buf[11] = self.port_disable_bus.into_bytes()[0];
            buf[12] = (self.max_power_self_ma / 2) as u8;
            buf[13] = (self.max_power_bus_ma / 2) as u8;
            buf[14] = (self.hub_current_self_ma / 2) as u8;
            buf[15] = (self.hub_current_bus_ma / 2) as u8;
            buf[16] = (self.power_on_time_ms / 2) as u8;
            buf[17] = (self.language_id >> 8) as u8;
            buf[18] = self.language_id as u8;
            buf[19] = self.manufacturer_string.byte_len();
            buf[20] = self.product_string.byte_len();
            buf[21] = self.serial_string.byte_len();
            chunks[0].0 = REG_VENDOR_ID_LSB;
            chunks[0].2 = 22;
        }

        // Chunks 1-2: Manufacturer string (0x16, 62 bytes max -> 32 + 30)
        let (a, b) = string_chunks(&self.manufacturer_string, REG_MANUFACTURER_STRING);
        chunks[1] = a;
        chunks[2] = b;

        // Chunks 3-4: Product string (0x54, 62 bytes max -> 32 + 30)
        let (a, b) = string_chunks(&self.product_string, REG_PRODUCT_STRING);
        chunks[3] = a;
        chunks[4] = b;

        // Chunks 5-6: Serial string (0x92, 62 bytes max -> 32 + 30)
        let (a, b) = string_chunks(&self.serial_string, REG_SERIAL_STRING);
        chunks[5] = a;
        chunks[6] = b;

        // Chunk 7: Battery charging (0xD0, 1 byte)
        chunks[7].0 = REG_BATTERY_CHARGING;
        chunks[7].1[0] = self.battery_charging.into_bytes()[0];
        chunks[7].2 = 1;

        // Chunk 8: Boost upstream (0xF6, 1 byte)
        chunks[8].0 = REG_BOOST_UPSTREAM;
        chunks[8].1[0] = self.boost_upstream.into_bytes()[0];
        chunks[8].2 = 1;

        // Chunk 9: Boost downstream (0xF8, 1 byte)
        chunks[9].0 = REG_BOOST_DOWNSTREAM;
        chunks[9].1[0] = self.boost_downstream.into_bytes()[0];
        chunks[9].2 = 1;

        // Chunk 10: Port swap (0xFA, 1 byte)
        // Chunk 11: Port maps (0xFB-0xFC split — but they're contiguous, so 0xFA-0xFC as 3 bytes)
        // Actually let's do port_swap + port_maps as one 3-byte chunk
        chunks[10].0 = REG_PORT_SWAP;
        chunks[10].1[0] = self.port_swap.into_bytes()[0];
        chunks[10].1[1] = self.port_map_12.into_bytes()[0];
        chunks[10].1[2] = self.port_map_34.into_bytes()[0];
        chunks[10].2 = 3;

        // Chunk 11 is unused — zero length signals "skip" to the driver
        chunks[11].2 = 0;

        chunks
    }
}

fn string_chunks(desc: &StringDescriptor, base_reg: u8) -> (Chunk, Chunk) {
    let mut full = [0u8; 62];
    let total = desc.write_le_bytes(&mut full);

    let mut chunk_a: Chunk = (base_reg, [0u8; 32], 0);
    let first = total.min(32);
    chunk_a.1[..first].copy_from_slice(&full[..first]);
    chunk_a.2 = first as u8;

    let mut chunk_b: Chunk = (base_reg.wrapping_add(32), [0u8; 32], 0);
    let second = total.saturating_sub(32);
    chunk_b.1[..second].copy_from_slice(&full[32..32 + second]);
    chunk_b.2 = second as u8;

    (chunk_a, chunk_b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::register::Variant;

    type TestResult<T> = Result<T, StringDescriptorError>;

    #[test]
    fn string_descriptor_empty() {
        let sd = StringDescriptor::empty();
        assert_eq!(sd.len(), 0);
        assert_eq!(sd.byte_len(), 0);
        assert!(sd.is_empty());
    }

    #[test]
    fn string_descriptor_default_is_empty() {
        let sd = StringDescriptor::default();
        assert_eq!(sd.len(), 0);
        assert!(sd.is_empty());
    }

    #[test]
    fn string_descriptor_encode_ascii() -> TestResult<()> {
        let sd = StringDescriptor::encode("Hello")?;
        assert_eq!(sd.len(), 5);
        assert_eq!(sd.byte_len(), 10);
        assert!(!sd.is_empty());

        let mut buf = [0u8; 10];
        let n = sd.write_le_bytes(&mut buf);
        assert_eq!(n, 10);
        // "Hello" as UTF-16LE
        assert_eq!(buf, [b'H', 0, b'e', 0, b'l', 0, b'l', 0, b'o', 0]);
        Ok(())
    }

    #[test]
    fn string_descriptor_encode_non_bmp() -> TestResult<()> {
        // U+1F600 (😀) encodes as a surrogate pair: 2 UTF-16 code units
        let sd = StringDescriptor::encode("😀")?;
        assert_eq!(sd.len(), 2);
        assert_eq!(sd.byte_len(), 4);
        Ok(())
    }

    #[test]
    fn string_descriptor_too_long() {
        let long = "a]".repeat(32); // 32 code units > 31 max
        let result = StringDescriptor::encode(&long);
        assert!(result.is_err());
    }

    #[test]
    fn string_descriptor_max_length() -> TestResult<()> {
        let max = "x".repeat(31);
        let sd = StringDescriptor::encode(&max)?;
        assert_eq!(sd.len(), 31);
        assert_eq!(sd.byte_len(), 62);
        Ok(())
    }

    #[test]
    fn for_variant_sets_product_id() {
        let c2 = Config::for_variant(Variant::Usb2512b);
        let c3 = Config::for_variant(Variant::Usb2513b);
        let c4 = Config::for_variant(Variant::Usb2514b);
        assert_eq!(c2.product_id, 0x2512);
        assert_eq!(c3.product_id, 0x2513);
        assert_eq!(c4.product_id, 0x2514);
        // Common defaults
        assert_eq!(c4.vendor_id, 0x0424);
        assert_eq!(c4.device_id, 0x0BB3);
    }

    #[test]
    fn register_chunks_count_and_addresses() {
        let config = Config::for_variant(Variant::Usb2514b);
        let chunks = config.to_register_chunks();

        // Chunk 0: IDs + config (0x00, 22 bytes)
        assert_eq!(chunks[0].0, 0x00);
        assert_eq!(chunks[0].2, 22);

        // Chunks 1-2: Manufacturer string
        assert_eq!(chunks[1].0, REG_MANUFACTURER_STRING);
        assert_eq!(chunks[2].0, REG_MANUFACTURER_STRING.wrapping_add(32));

        // Chunks 3-4: Product string
        assert_eq!(chunks[3].0, REG_PRODUCT_STRING);

        // Chunks 5-6: Serial string
        assert_eq!(chunks[5].0, REG_SERIAL_STRING);

        // Chunk 7: Battery charging
        assert_eq!(chunks[7].0, REG_BATTERY_CHARGING);
        assert_eq!(chunks[7].2, 1);

        // Chunk 8: Boost upstream
        assert_eq!(chunks[8].0, REG_BOOST_UPSTREAM);
        assert_eq!(chunks[8].2, 1);

        // Chunk 9: Boost downstream
        assert_eq!(chunks[9].0, REG_BOOST_DOWNSTREAM);
        assert_eq!(chunks[9].2, 1);

        // Chunk 10: Port swap + maps (3 bytes)
        assert_eq!(chunks[10].0, REG_PORT_SWAP);
        assert_eq!(chunks[10].2, 3);

        // Chunk 11: unused
        assert_eq!(chunks[11].2, 0);
    }

    #[test]
    fn register_chunks_ids_serialized_le() {
        let config = Config::for_variant(Variant::Usb2514b);
        let chunks = config.to_register_chunks();
        let buf = &chunks[0].1;

        // Vendor ID 0x0424 little-endian
        assert_eq!(buf[0], 0x24);
        assert_eq!(buf[1], 0x04);
        // Product ID 0x2514 little-endian
        assert_eq!(buf[2], 0x14);
        assert_eq!(buf[3], 0x25);
        // Device ID 0x0BB3 little-endian
        assert_eq!(buf[4], 0xB3);
        assert_eq!(buf[5], 0x0B);
        // Config byte 1: 0x9B
        assert_eq!(buf[6], 0x9B);
        // Config byte 2: 0x20
        assert_eq!(buf[7], 0x20);
    }

    #[test]
    fn register_chunks_power_fields() {
        let mut config = Config::for_variant(Variant::Usb2514b);
        config.max_power_bus_ma = 500;
        config.power_on_time_ms = 200;

        let chunks = config.to_register_chunks();
        let buf = &chunks[0].1;

        // 500mA / 2 = 250
        assert_eq!(buf[13], 250);
        // 200ms / 2 = 100
        assert_eq!(buf[16], 100);
    }

    #[test]
    fn register_chunks_string_lengths() -> TestResult<()> {
        let mut config = Config::for_variant(Variant::Usb2514b);
        config.manufacturer_string = StringDescriptor::encode("Test")?;

        let chunks = config.to_register_chunks();
        let buf = &chunks[0].1;

        // Manufacturer string: 4 codeunits * 2 = 8 bytes
        assert_eq!(buf[19], 8);
        // Product + serial: empty = 0
        assert_eq!(buf[20], 0);
        assert_eq!(buf[21], 0);

        // First string chunk should have 8 bytes of data
        assert_eq!(chunks[1].2, 8);
        // Second chunk empty (8 < 32)
        assert_eq!(chunks[2].2, 0);
        Ok(())
    }

    #[test]
    fn register_chunks_long_string_splits() -> TestResult<()> {
        let mut config = Config::for_variant(Variant::Usb2514b);
        // 20 chars = 40 bytes UTF-16LE → split into 32 + 8
        config.manufacturer_string = StringDescriptor::encode("12345678901234567890")?;

        let chunks = config.to_register_chunks();
        assert_eq!(chunks[1].2, 32);
        assert_eq!(chunks[2].2, 8);
        Ok(())
    }
}
